use crate::errors::{DkimError, ToDkimError};
use crate::utils::collapse_whitespace;
use mailparse::parse_mail;
use std::collections::HashMap;
use trust_dns_resolver::TokioAsyncResolver;

/// Fetches the DKIM public key (p=) from DNS based on the DKIM-Signature
/// in the given raw email.
///
/// This is required to verify DKIM signatures. Our login/ZK flow relies on
/// retrieving the public key for signature verification.
///
/// Steps:
/// 1. Parse the raw email headers using `mailparse`.
/// 2. Locate the first `DKIM-Signature` header.
/// 3. Parse DKIM parameters (d=domain, s=selector) from the header.
/// 4. Build the DNS TXT query: `<selector>._domainkey.<domain>`.
/// 5. Lookup TXT records using async DNS resolver.
/// 6. Parse TXT data to find the `p=` public key.
///
/// Notes:
/// - Only the first DKIM-Signature header is used (multi-signature emails ignored).
/// - Folding, extra whitespace, and line breaks are normalized.
/// - Returns an error if `DKIM-Signature`, `d=`, `s=`, or `p=` is missing.
pub async fn dkim_key_fetch(raw_email: String) -> Result<String, DkimError> {
    // Parse email headers
    let parsed = parse_mail(raw_email.as_bytes()).to_dkim_key_fetch_err()?;
    let headers = parsed.get_headers();

    // Find DKIM-Signature header
    let dkim_header = headers
        .into_iter()
        .find(|h| h.get_key().eq_ignore_ascii_case("DKIM-Signature"))
        .map(|h| h.get_value())
        .ok_or_else(|| {
            DkimError::DkimKeyFetch("No DKIM-Signature header found in message.".into())
        })?;

    // Parse DKIM parameters
    let params = parse_dkim_header_params(&dkim_header);
    let domain = params.get("d").ok_or_else(|| {
        DkimError::DkimKeyFetch("DKIM header missing 'd=' domain parameter.".into())
    })?;
    let selector = params.get("s").ok_or_else(|| {
        DkimError::DkimKeyFetch("DKIM header missing 's=' selector parameter.".into())
    })?;

    let dns_name = format!("{}._domainkey.{}", selector, domain);

    // Async DNS resolver
    let resolver = TokioAsyncResolver::tokio_from_system_conf().to_dkim_key_fetch_err()?;
    let txt_response = resolver
        .txt_lookup(dns_name.clone())
        .await
        .to_dkim_key_fetch_err()?;

    // Extract p= from TXT records
    for txt in txt_response.iter() {
        let joined = txt
            .txt_data()
            .iter()
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .collect::<Vec<_>>()
            .join("");

        let txt_params = parse_tag_value_pairs(&joined);
        if let Some(pub_key) = txt_params.get("p") {
            return Ok(pub_key.to_string());
        }
    }

    Err(DkimError::DkimKeyFetch(format!(
        "No 'p=' public key found in TXT records for {}.",
        dns_name
    )))
}

/// Parses a DKIM-Signature header into key-value parameters.
///
/// Example: `"v=1; a=rsa-sha256; d=example.com; s=brisbane;"`
/// becomes `{"v":"1", "a":"rsa-sha256", "d":"example.com", "s":"brisbane"}`
pub fn parse_dkim_header_params(header: &str) -> HashMap<String, String> {
    parse_tag_value_pairs(header)
}

/// Generic parser for `key=value` pairs separated by `;`, tolerant to
/// whitespace and line breaks. Normalizes whitespace using `collapse_whitespace`.
pub fn parse_tag_value_pairs(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let norm = collapse_whitespace(&s.replace("\r\n", " ").replace('\n', " "));

    for part in norm.split(';') {
        let p = part.trim();
        if let Some(eq_pos) = p.find('=') {
            let key = p[..eq_pos].trim().to_lowercase();
            let val = p[eq_pos + 1..].trim().to_string();
            map.insert(key, val);
        }
    }

    map
}
