//! Minimal DKIM verification (RSA-SHA256) with canonicalization (relaxed/simple).
//! Includes unit tests that generate signed emails and verify them.

use base64::{Engine as _, engine::general_purpose};
use mailparse::parse_mail;
use pkcs1::DecodeRsaPublicKey;
use rsa::RsaPrivateKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Sign, RsaPublicKey};
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Algorithm enum (only RSA implemented here)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DkimAlgorithm {
    RsaSha256,
    // EcdsaSha256, // future
}

/// Errors
#[derive(Error, Debug)]
pub enum DkimError {
    #[error("no DKIM-Signature header")]
    NoDkimSignature,
    #[error("invalid signature field")]
    BadSignatureField,
    #[error("body hash mismatch")]
    BodyHashMismatch,
    #[error("signature verification failed")]
    SignatureVerifyFailed,
    #[error("public key parse error: {0}")]
    PublicKeyParse(String),
    #[error("other: {0}")]
    Other(String),
}

/// Structure for parsed DKIM-Signature header fields we need
#[derive(Debug, Clone)]
struct DkimSignatureFields {
    v: Option<String>,  // algorithm
    a: Option<String>,  // version
    c: Option<String>,  // canonicalization (header/body)
    d: Option<String>,  // domain
    s: Option<String>,  // selector
    h: Option<String>,  // signed header list
    bh: Option<String>, // body hash (base64)
    b: Option<String>,  // signature (base64)
}

impl DkimSignatureFields {
    fn get_b(&self) -> Option<String> {
        self.b.clone()
    }
}

/// Parse DKIM-Signature header
fn parse_dkim_signature_header(value: &str) -> Result<DkimSignatureFields, DkimError> {
    let mut fields = DkimSignatureFields {
        v: None,
        a: None,
        c: None,
        d: None,
        s: None,
        h: None,
        bh: None,
        b: None,
    };
    let mut v = value.trim();
    if v.to_lowercase().starts_with("dkim-signature:") {
        v = &v["dkim-signature:".len()..];
    }
    for part in v.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(eq) = part.find('=') {
            let (k, val) = (&part[..eq], &part[eq + 1..]);
            match k {
                "v" => fields.v = Some(val.to_string()),
                "a" => fields.a = Some(val.to_string()),
                "c" => fields.c = Some(val.to_string()),
                "d" => fields.d = Some(val.to_string()),
                "s" => fields.s = Some(val.to_string()),
                "h" => fields.h = Some(val.to_string()),
                "bh" => fields.bh = Some(val.to_string()),
                "b" => fields.b = Some(val.to_string()),
                _ => {}
            }
        }
    }
    Ok(fields)
}

/// DKIM canonicalization (header relaxed)
fn canonicalize_header_relaxed(name: &str, value: &str) -> String {
    let name_l = name.to_lowercase();
    let mut out = String::new();
    let mut prev_wsp = false;
    for ch in value.replace("\r\n", "").chars() {
        if ch == ' ' || ch == '\t' {
            if !prev_wsp {
                out.push(' ');
                prev_wsp = true;
            }
        } else {
            out.push(ch);
            prev_wsp = false;
        }
    }
    format!("{}: {}", name_l, out.trim())
}

/// Body canonicalization simple
fn canonicalize_body_simple(body: &str) -> String {
    let mut lines: Vec<&str> = body
        .replace("\r\n", "\n")
        .replace("\r", "\n")
        .split('\n')
        .collect();
    while matches!(lines.last(), Some(l) if l.is_empty()) {
        lines.pop();
    }
    if lines.is_empty() {
        "".to_string()
    } else {
        lines.join("\r\n") + "\r\n"
    }
}

/// Body canonicalization relaxed
fn canonicalize_body_relaxed(body: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    for line in body.replace("\r\n", "\n").replace("\r", "\n").split('\n') {
        let mut out = String::new();
        let mut prev_wsp = false;
        for ch in line.chars() {
            if ch == ' ' || ch == '\t' {
                if !prev_wsp {
                    out.push(' ');
                    prev_wsp = true;
                }
            } else {
                out.push(ch);
                prev_wsp = false;
            }
        }
        lines.push(out.trim_end().to_string());
    }
    while matches!(lines.last(), Some(l) if l.is_empty()) {
        lines.pop();
    }
    if lines.is_empty() {
        "".to_string()
    } else {
        lines.join("\r\n") + "\r\n"
    }
}

/// Compute body hash
fn compute_body_hash_b64(body: &str, body_can: &str) -> String {
    let canonical = match body_can {
        "simple" => canonicalize_body_simple(body),
        "relaxed" => canonicalize_body_relaxed(body),
        _ => canonicalize_body_simple(body),
    };
    let digest = Sha256::digest(canonical.as_bytes());
    general_purpose::STANDARD.encode(digest)
}

/// Build signed headers string
fn build_signed_headers_string(
    headers: &[(String, String)],
    _dkim_index: usize,
    h_list: &str,
    header_can: &str,
) -> String {
    let want: Vec<&str> = h_list.split(':').map(|s| s.trim()).collect();
    let mut out = String::new();
    for name in want {
        if let Some((hname, hvalue)) = headers
            .iter()
            .rev()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
        {
            let v = if hname.eq_ignore_ascii_case("DKIM-Signature") {
                let mut s = hvalue.clone();
                if let Some(pos) = s.to_lowercase().find("b=") {
                    s.truncate(pos + 2);
                }
                s
            } else {
                hvalue.clone()
            };
            out.push_str(&match header_can {
                "relaxed" => canonicalize_header_relaxed(hname, &v),
                _ => format!("{}: {}\r\n", hname, v),
            });
        }
    }
    out
}

/// Verify DKIM signature (RSA-SHA256)
pub fn verify_dkim_signature_rsa(raw_email: &str, pubkey_pem: &str) -> Result<(), DkimError> {
    let parsed = parse_mail(raw_email.as_bytes())
        .map_err(|e| DkimError::Other(format!("mailparse: {:?}", e)))?;
    let headers: Vec<(String, String)> = parsed
        .get_headers()
        .iter()
        .map(|h| (h.get_key().to_string(), h.get_value().to_string()))
        .collect();
    let body = parsed
        .get_body()
        .map_err(|e| DkimError::Other(format!("body parse: {:?}", e)))?;

    // DKIM header
    let (dkim_idx, dkim_raw) = headers
        .iter()
        .enumerate()
        .find(|(_, (n, _))| n.eq_ignore_ascii_case("DKIM-Signature"))
        .ok_or(DkimError::NoDkimSignature)?;
    let dkim_fields = parse_dkim_signature_header(&dkim_raw)?;

    let bh = dkim_fields.bh.ok_or(DkimError::BadSignatureField)?;
    let b_sig = dkim_fields.b.ok_or(DkimError::BadSignatureField)?;
    let h_list = dkim_fields.h.ok_or(DkimError::BadSignatureField)?;
    let c = dkim_fields
        .c
        .unwrap_or_else(|| "relaxed/simple".to_string());
    let parts: Vec<&str> = c.split('/').collect();
    let header_can = parts.get(0).unwrap_or(&"relaxed");
    let body_can = parts.get(1).unwrap_or(&"simple");

    // verify body hash
    if compute_body_hash_b64(&body, body_can) != bh {
        return Err(DkimError::BodyHashMismatch);
    }

    let signed_headers_str = build_signed_headers_string(&headers, *dkim_idx, &h_list, header_can);

    let sig_bytes = general_purpose::STANDARD
        .decode(&b_sig)
        .map_err(|e| DkimError::Other(format!("base64 decode sig: {}", e)))?;

    let pubkey = RsaPublicKey::from_pkcs1_pem(pubkey_pem)
        .or_else(|_| RsaPublicKey::from_public_key_pem(pubkey_pem))
        .map_err(|e| DkimError::PublicKeyParse(format!("{:?}", e)))?;

    let digest = Sha256::digest(signed_headers_str.as_bytes());
    let padding = PaddingScheme::PKCS1v15Sign {
        hash: Some(rsa::hash::Hash::SHA2_256),
    };
    pubkey
        .verify(padding, &digest, &sig_bytes)
        .map_err(|_| DkimError::SignatureVerifyFailed)
}

/// Helper test-only routine: create DKIM signature value for given headers/body using RSA private key.
/// This constructs a DKIM-Signature header (without folding) with b=... computed.
/// Returns the header string value (i.e., the full DKIM-Signature: ... value)
#[cfg(test)]
fn sign_dkim_headers_rsa(
    headers: &[(String, String)],
    body: &str,
    selector: &str,
    domain: &str,
    priv_key: &RsaPrivateKey,
    header_can: &str,
    body_can: &str,
) -> String {
    // compute bh
    let bh = compute_body_hash_b64(body, body_can);

    // construct initial DKIM-Signature header with empty b=
    // include tags v=1; a=rsa-sha256; c=header_can/body_can; d=domain; s=selector; h=...; bh=...; b=
    let h_list = headers
        .iter()
        .map(|(n, _v)| n.as_str())
        .collect::<Vec<&str>>()
        .join(":");
    let dkim_value_prefix = format!(
        "v=1; a=rsa-sha256; c={}/{}; d={}; s={}; h={}; bh={}; b=",
        header_can, body_can, domain, selector, h_list, bh
    );

    // DKIM signs the canonicalized header block per h= list with the DKIM-Signature header's b= empty value
    // Build signed headers string using canonicalization
    // For signing, we need the headers vector to include the DKIM-Signature header itself with b= empty.
    let mut headers_for_sign = headers.to_vec();
    headers_for_sign.push(("DKIM-Signature".to_string(), dkim_value_prefix.clone()));

    let signed_headers_str = build_signed_headers_string(
        &headers_for_sign,
        headers_for_sign.len() - 1,
        &h_list,
        header_can,
    );

    // sign signed_headers_str with RSA PKCS1v15 SHA256
    let mut hasher = Sha256::new();
    hasher.update(signed_headers_str.as_bytes());
    let digest = hasher.finalize();

    let padding = PaddingScheme::PKCS1v15Sign {
        hash: Some(rsa::hash::Hash::SHA2_256),
    };
    let signature = priv_key.sign(padding, &digest).expect("sign");

    let b64 = general_purpose::STANDARD.encode(&signature);

    let full = format!("{}{}", dkim_value_prefix, b64);
    full
}

#[cfg(test)]
mod tests {
    use super::*;
    use pem::Pem;
    use rand::rngs::OsRng;
    use rsa::pkcs1::ToRsaPrivateKey;
    use rsa::pkcs1::ToRsaPublicKey;

    /// Build a small sample email given headers and body
    fn build_raw_email(headers: &[(String, String)], body: &str) -> String {
        let mut out = String::new();
        for (n, v) in headers {
            out.push_str(&format!("{}: {}\r\n", n, v));
        }
        out.push_str("\r\n");
        out.push_str(body);
        out
    }

    #[test]
    fn test_rsa_dkim_sign_and_verify_simple_relaxed() {
        // generate RSA keypair
        let mut rng = OsRng;
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("generate key");
        let pub_key = RsaPublicKey::from(&priv_key);

        // convert pubkey to PEM (PKCS1)
        let pub_pem = pub_key.to_pkcs1_pem(Default::default()).expect("pub pem");
        let priv_pem = priv_key.to_pkcs1_pem(Default::default()).expect("priv pem");

        // headers (note order matters). We'll include From, To, Subject, MIME-Version, Content-Type, Message-ID, Date
        let headers = vec![
            ("From".to_string(), "Alice <alice@example.com>".to_string()),
            ("To".to_string(), "bob@example.com".to_string()),
            ("Subject".to_string(), "Test DKIM".to_string()),
            ("MIME-Version".to_string(), "1.0".to_string()),
            (
                "Content-Type".to_string(),
                "text/plain; charset=UTF-8".to_string(),
            ),
            ("Message-ID".to_string(), "<msgid@example.com>".to_string()),
            (
                "Date".to_string(),
                "Wed, 05 Nov 2025 17:35:02 +0000".to_string(),
            ),
        ];
        let body = "Hello Bob,\r\nThis is a DKIM test.\r\n\r\nRegards,\r\nAlice\r\n";

        // choose canonicalization
        let header_can = "relaxed";
        let body_can = "simple";

        // compute DKIM header value (signed)
        let dkim_value = sign_dkim_headers_rsa(
            &headers,
            body,
            "sel",
            "example.com",
            &priv_key,
            header_can,
            body_can,
        );

        // create raw email with DKIM-Signature header inserted near top
        let mut headers_with_dkim = vec![("DKIM-Signature".to_string(), dkim_value.clone())];
        headers_with_dkim.extend(headers.clone());

        let raw_email = build_raw_email(&headers_with_dkim, body);

        // Now verify using the public key PEM
        let res = verify_dkim_signature_rsa(&raw_email, &pub_pem);
        assert!(res.is_ok(), "verification should succeed: {:?}", res.err());

        // Tamper body -> should fail body hash
        let mut tampered = raw_email.clone();
        tampered = tampered.replacen("DKIM test.", "DKIM test (tampered).", 1);
        let res2 = verify_dkim_signature_rsa(&tampered, &pub_pem);
        assert!(matches!(res2, Err(DkimError::BodyHashMismatch)));

        // Tamper signature by changing public key -> should fail signature verify
        // generate different key
        let other_priv = RsaPrivateKey::new(&mut rng, bits).expect("other");
        let other_pub = RsaPublicKey::from(&other_priv);
        let other_pub_pem = other_pub
            .to_pkcs1_pem(Default::default())
            .expect("other pub pem");
        let res3 = verify_dkim_signature_rsa(&raw_email, &other_pub_pem);
        assert!(matches!(res3, Err(DkimError::SignatureVerifyFailed)));
    }

    #[test]
    fn test_canonicalization_relaxed_examples() {
        // header relaxed canonicalization test
        let name = "Subject";
        let value = "  This   is   a   test  \r\n\t  with folding ";
        let canon = canonicalize_header_relaxed(name, value);
        // should compress spaces and lower-case name and include single leading space before value
        assert!(canon.starts_with("subject:"), "got {}", canon);
        assert!(canon.contains("This is a test"), "got {}", canon);

        // body relaxed canonicalization test
        let body = "Line with   spaces  \r\nSecond\tline\r\n\r\n\r\n";
        let crel = canonicalize_body_relaxed(body);
        assert!(crel.ends_with("\r\n"));
    }
}
