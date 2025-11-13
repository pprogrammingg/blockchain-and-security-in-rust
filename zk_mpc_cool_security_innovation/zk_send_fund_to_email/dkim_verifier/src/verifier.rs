use crate::errors::{DkimError, ToDkimError};
use base64;
use base64::Engine;
use base64::engine::general_purpose;
use regex::Regex;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::signature::DigestVerifier;
use rsa::{RsaPublicKey, pkcs1::DecodeRsaPublicKey};
use sha2::{Digest, Sha256};
/// Verify DKIM signature of a raw email given the public key in PEM or DER format.
pub fn verify_email(
    raw_email: &str,
    public_key_pem: &str,
    dkim_params: &std::collections::HashMap<String, String>,
) -> Result<bool, DkimError> {
    // get the relative params from header
    let bh_field = dkim_params
        .get("bh")
        .ok_or(DkimError::DkimVerifier("Missing header field 'bh'".into()))?;
    let b_field = dkim_params
        .get("b")
        .ok_or(DkimError::DkimVerifier("Missing header field 'b'".into()))?;
    let canon_type = dkim_params
        .get("c")
        .map(|s| s.as_str())
        .unwrap_or("simple/simple");

    let body = extract_body(raw_email)?;
    let canon_body = canonicalize_body(&body, canon_type)?;

    let mut hasher_body = Sha256::new(); // Renamed to avoid conflict later
    hasher_body.update(canon_body.as_bytes());
    let digest_bytes = hasher_body.finalize();
    let computed_bh = general_purpose::STANDARD.encode(digest_bytes);
    if computed_bh != *bh_field {
        return Err(DkimError::DkimVerifier(format!(
            "Body hash mismatch, expected: {}, found {}",
            bh_field.clone(),
            computed_bh
        )));
    }

    // Canonicalize signed headers
    let h_list = dkim_params
        .get("h")
        .ok_or(DkimError::DkimVerifier("Missing header field 'h'".into()))?;
    let headers_canon = canonicalize_headers(raw_email, h_list, canon_type)?;

    // Decode signature bytes (as a raw Vec<u8>)
    let sig_bytes_vec = general_purpose::STANDARD
        .decode(b_field)
        .to_dkim_verifier_err()?;

    // Parse RSA public key
    let pub_key = RsaPublicKey::from_pkcs1_pem(public_key_pem).to_dkim_verifier_err()?;

    // Convert the raw Vec<u8> into the required `rsa::pkcs1v15::Signature` struct
    let signature = Signature::try_from(sig_bytes_vec.as_slice()).to_dkim_verifier_err()?;

    // Verify signature

    // Create a new hasher instance for the headers *before* finalization
    let mut hasher_headers = Sha256::new();
    hasher_headers.update(headers_canon.as_bytes());

    // Create a verifying key with the correct digest type (Sha256)
    let verifying_key = VerifyingKey::new_unprefixed(pub_key);

    // The `verify_digest` method expects the *hasher instance* (D: Digest) and a `&Signature` struct.
    verifying_key
        .verify_digest(hasher_headers, &signature) // Pass hasher by value, signature by ref
        .to_dkim_verifier_err()?;

    Ok(true)
}

/// Simple canonicalization of body according to DKIM rules
fn canonicalize_body(body: &str, canon_type: &str) -> Result<String, DkimError> {
    match canon_type {
        "relaxed/relaxed" => {
            let mut lines: Vec<String> = body
                .lines()
                .map(|l| l.trim_end().replace("\t", " "))
                .collect();
            // remove trailing empty lines
            while matches!(lines.last(), Some(l) if l.is_empty()) {
                lines.pop();
            }
            Ok(lines.join("\r\n"))
        }
        _ => Ok(body.to_string()), // simple/simple
    }
}

/// Canonicalize headers according to DKIM spec
fn canonicalize_headers(
    raw_email: &str,
    header_list: &str,
    canon_type: &str,
) -> Result<String, DkimError> {
    use mailparse::parse_mail;

    let parsed = parse_mail(raw_email.as_bytes()).to_dkim_verifier_err()?;
    let mut result = String::new();

    for header_name in header_list.split(':') {
        let header_name = header_name.trim();
        if let Some(header) = parsed
            .get_headers()
            .into_iter()
            .find(|h| h.get_key().eq_ignore_ascii_case(header_name))
        {
            let mut val = header.get_value();

            if canon_type == "relaxed/relaxed" {
                let re = Regex::new(r"\s+").unwrap();
                val = re.replace_all(val.trim(), " ").to_string();
            }
            result.push_str(&format!("{}:{}", header_name.to_lowercase(), val));
            result.push_str("\r\n");
        } else {
            return Err(DkimError::DkimVerifier(format!(
                "Missing header : {}",
                String::from(header_name)
            )));
        }
    }
    Ok(result)
}

/// Extract body portion of email
fn extract_body(raw_email: &str) -> Result<String, DkimError> {
    use mailparse::parse_mail;
    let parsed = parse_mail(raw_email.as_bytes()).to_dkim_verifier_err()?;
    parsed.get_body().to_dkim_verifier_err()
}
