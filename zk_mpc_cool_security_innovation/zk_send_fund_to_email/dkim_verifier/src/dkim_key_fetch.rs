use crate::errors::{DkimError, ToDkimError};
use std::{env, fs};
use trust_dns_resolver::TokioAsyncResolver;

#[tokio::main]
async fn main() -> Result<(), DkimError> {
    // Accept file path or read stdin if - passed
    let arg = env::args().nth(1).unwrap_or_else(|| "-".to_string());
    let raw = if arg == "-" {
        // read from stdin
        let mut s = String::new();
        std::io::read_to_string(std::io::stdin()).to_dkim_key_fetch_err()?;
        s
    } else {
        fs::read_to_string(&arg).to_dkim_key_fetch_err()?
    };
    println!("{}", arg);

    // Parse message headers
    let parsed = mailparse::parse_mail(&raw.as_bytes()).to_dkim_key_fetch_err()?;
    // mailparse returns headers in parsed.get_headers()
    let headers = parsed.get_headers();

    // Find DKIM-Signature header (there can be multiple; we'll handle first)
    let dkim_header = headers
        .into_iter()
        .find(|h| h.get_key().eq_ignore_ascii_case("DKIM-Signature"))
        .map(|h| h.get_value());

    let dkim_raw = match dkim_header {
        Some(v) => v,
        None => {
            println!("No DKIM-Signature header found in message.");
            return Ok(());
        }
    };

    println!("Found DKIM-Signature header:\n{}\n", dkim_raw);

    // Parse DKIM params: "k=v; k2=v2; ..." -- header may contain folded whitespace
    let params = parse_dkim_header_params(&dkim_raw);
    // Need domain (d=) and selector (s=)
    let domain = match params.get("d") {
        Some(d) => d,
        None => {
            println!("DKIM header missing 'd=' domain parameter.");
            return Ok(());
        }
    };
    let selector = match params.get("s") {
        Some(s) => s,
        None => {
            println!("DKIM header missing 's=' selector parameter.");
            return Ok(());
        }
    };

    println!("DKIM domain (d): {}", domain);
    println!("DKIM selector (s): {}", selector);

    // Build DNS query name: "<selector>._domainkey.<domain>"
    let dns_name = format!("{}._domainkey.{}", selector, domain);
    println!("Querying TXT record for: {}\n", dns_name);

    // Create resolver with system config
    let resolver = TokioAsyncResolver::tokio_from_system_conf().to_dkim_key_fetch_err()?;

    // Do TXT lookup
    let txt_response = resolver
        .txt_lookup(dns_name.clone())
        .await
        .to_dkim_key_fetch_err()?;

    // TXT records may be split across multiple strings; join each txt data into one long string.
    let mut found = false;
    for txt in txt_response.iter() {
        let joined = txt
            .txt_data()
            .iter()
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .collect::<Vec<_>>()
            .join("");
        println!("DNS Raw TXT record: {}\n", joined);

        // Parse params of the TXT (format like: "v=DKIM1; k=rsa; p=MIIBI...;")
        let txt_params = parse_tag_value_pairs(&joined);
        if let Some(p) = txt_params.get("p") {
            println!("Found public key (p=):\n{}\n", p);
            found = true;
        } else {
            println!("TXT record didn't contain p= parameter.\n");
        }
    }

    if !found {
        println!("No 'p=' public key found in TXT records for {}.", dns_name);
    }

    Ok(())
}

// Parse a DKIM header value into a map of params.
// Example header body: "v=1; a=rsa-sha256; d=example.com; s=brisbane; ..."
// This is tolerant to whitespace and folded lines.
fn parse_dkim_header_params(header: &str) -> std::collections::HashMap<String, String> {
    // DKIM headers sometimes include the "DKIM-Signature: " prefix if raw; ensure we only parse value.
    // We simply split on ';' and then split first '='.
    parse_tag_value_pairs(header)
}

fn parse_tag_value_pairs(s: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    // Remove line breaks and normalize spaces
    let norm = s.replace("\r\n", " ").replace('\n', " ");
    // Split by ';'
    for part in norm.split(';') {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        if let Some(eq_pos) = p.find('=') {
            let key = p[..eq_pos].trim().to_lowercase();
            let val = p[eq_pos + 1..].trim().to_string();
            map.insert(key, val);
        }
    }
    map
}
