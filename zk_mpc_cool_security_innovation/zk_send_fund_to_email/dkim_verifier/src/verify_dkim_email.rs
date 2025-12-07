use base64::{Engine as _, engine::general_purpose};
use mailparse::{MailHeaderMap, parse_mail};
use regex::Regex;
use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::signature::{SignatureEncoding, Signer, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Canonicalize body according to DKIM rules
fn canonicalize_body(body: &str, canon_type: &str) -> String {
    match canon_type {
        "relaxed" => {
            let mut lines: Vec<String> = body
                .lines()
                .map(|l| l.trim_end().replace('\t', " "))
                .collect();
            while matches!(lines.last(), Some(l) if l.is_empty()) {
                lines.pop();
            }
            lines.join("\r\n")
        }
        _ => body.to_string(),
    }
}

/// Canonicalize headers according to DKIM rules
fn canonicalize_headers(
    raw_email: &str,
    header_list: &[&str],
    canon_type: &str,
) -> Result<String, String> {
    let parsed = parse_mail(raw_email.as_bytes()).map_err(|e| e.to_string())?;
    let headers = parsed.get_headers();

    let mut result = String::new();

    // Compile regex once if relaxed canonicalization is used
    let whitespace_re = if canon_type == "relaxed" {
        Some(Regex::new(r"\s+").unwrap())
    } else {
        None
    };

    for &header_name in header_list {
        if let Some(h) = headers.get_first_header(header_name) {
            let mut val = h.get_value();
            if let Some(re) = &whitespace_re {
                val = re.replace_all(val.trim(), " ").to_string();
            }
            result.push_str(&format!("{}:{}", header_name.to_lowercase(), val));
            result.push_str("\r\n");
        } else {
            return Err(format!("Missing header: {}", header_name));
        }
    }

    Ok(result)
}

/// Extract body from raw email
fn extract_body(raw_email: &str) -> Result<String, String> {
    let parsed = parse_mail(raw_email.as_bytes()).map_err(|e| e.to_string())?;
    parsed.get_body().map_err(|e| e.to_string())
}

/// Sign DKIM
pub fn sign_email(
    raw_email: &str,
    priv_key: &RsaPrivateKey,
    selector: &str,
    domain: &str,
    headers_to_sign: &[&str],
    canon_type: &str,
) -> Result<String, String> {
    let body = extract_body(raw_email)?;
    let canon_body = canonicalize_body(&body, canon_type);

    // Compute body hash
    let mut hasher = Sha256::new();
    hasher.update(canon_body.as_bytes());
    let bh = general_purpose::STANDARD.encode(hasher.finalize());

    // Build initial DKIM header (without b=)
    let dkim_prefix = format!(
        "v=1; a=rsa-sha256; c={0}/{0}; d={1}; s={2}; bh={3}; b=",
        canon_type, domain, selector, bh
    );

    // Combine DKIM header + email headers for signing
    let mut headers_for_sign = vec![("DKIM-Signature".to_string(), dkim_prefix.clone())];
    let parsed = parse_mail(raw_email.as_bytes()).map_err(|e| e.to_string())?;
    for h in parsed.get_headers() {
        headers_for_sign.push((h.get_key().to_string(), h.get_value()));
    }

    let signed_headers_str = canonicalize_headers(raw_email, headers_to_sign, canon_type)?;

    let digest = Sha256::digest(signed_headers_str.as_bytes());
    let signing_key = SigningKey::<Sha256>::new(priv_key.clone());
    let sig: Signature = signing_key.sign(&digest);

    let b64 = general_purpose::STANDARD.encode(sig.to_bytes());

    Ok(format!("{}{}", dkim_prefix, b64))
}

/// Verify DKIM
pub fn verify_email(
    raw_email: &str,
    pub_key: &RsaPublicKey,
    dkim_header: &str,
    headers_to_verify: &[&str],
    canon_type: &str,
) -> Result<(), String> {
    // Parse DKIM header for bh and b
    let mut dkim_map: HashMap<String, String> = HashMap::new();
    for part in dkim_header.split(';') {
        let kv = part.trim();
        if let Some((k, v)) = kv.split_once('=') {
            dkim_map.insert(k.to_string(), v.to_string());
        }
    }

    let bh = dkim_map
        .get("bh")
        .ok_or("Missing bh field in DKIM header")?;
    let b = dkim_map.get("b").ok_or("Missing b field in DKIM header")?;

    // Canonicalize and hash body
    let body = extract_body(raw_email)?;
    let canon_body = canonicalize_body(&body, canon_type);
    let digest_body = Sha256::digest(canon_body.as_bytes());
    let computed_bh = general_purpose::STANDARD.encode(digest_body);

    if &computed_bh != bh {
        return Err("Body hash mismatch".into());
    }

    // Canonicalize headers
    let headers_canon = canonicalize_headers(raw_email, headers_to_verify, canon_type)?;

    let digest_headers = Sha256::digest(headers_canon.as_bytes());

    // Decode signature
    let sig_bytes = general_purpose::STANDARD
        .decode(b)
        .map_err(|_| "Invalid base64 signature")?;
    let signature = Signature::try_from(sig_bytes.as_slice()).map_err(|_| "Invalid signature")?;

    let verifying_key = VerifyingKey::<Sha256>::new(pub_key.clone());
    verifying_key
        .verify(&digest_headers, &signature)
        .map_err(|_| "Signature verification failed".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    const RAW_EMAIL: &str =
        "From: alice@example.com\r\nTo: bob@example.com\r\nSubject: DKIM Test\r\n\r\nHello DKIM!";

    #[test]
    fn test_dkim_sign_verify() {
        let mut rng = OsRng;
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("keygen");
        let pub_key = RsaPublicKey::from(&priv_key);

        let headers_to_sign = ["from", "to", "subject", "DKIM-Signature"];
        let canon = "relaxed";

        let dkim_header = sign_email(
            RAW_EMAIL,
            &priv_key,
            "selector",
            "example.com",
            &headers_to_sign,
            canon,
        )
        .unwrap();

        // Verification
        assert!(verify_email(RAW_EMAIL, &pub_key, &dkim_header, &headers_to_sign, canon).is_ok());

        // Tamper -> should fail
        let tampered_email = RAW_EMAIL.replace("Hello DKIM!", "Hello tampered!");
        assert!(
            verify_email(
                &tampered_email,
                &pub_key,
                &dkim_header,
                &headers_to_sign,
                canon
            )
            .is_err()
        );
    }
}
