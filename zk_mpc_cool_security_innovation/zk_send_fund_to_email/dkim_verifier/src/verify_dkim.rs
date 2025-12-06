//! Minimal DKIM verification (RSA-SHA256) with canonicalization (relaxed/simple).
//! Includes unit tests that generate signed emails and verify them.

use base64::{Engine as _, engine::general_purpose};
use rsa::pkcs1::{FromRsaPrivateKey, FromRsaPublicKey};
use rsa::{PaddingScheme, RsaPrivateKey, RsaPublicKey};
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
    v: Option<String>,
    a: Option<String>,
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

/// Parse raw email into (headers_vec, body)
fn split_headers_body(raw: &str) -> (Vec<(String, String)>, String) {
    // split at first blank line (RFC822)
    let mut headers = Vec::new();
    let mut lines = raw.replace("\r\n", "\n").lines();
    let mut in_headers = true;
    let mut last_name: Option<String> = None;
    let mut last_value: Option<String> = None;
    let mut body_lines: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        if in_headers {
            if line.trim().is_empty() {
                // finish last header
                if let (Some(n), Some(v)) = (last_name.take(), last_value.take()) {
                    headers.push((n, v));
                }
                in_headers = false;
                // remaining lines are body (including this blank line omitted)
                for l in lines {
                    body_lines.push(l.to_string());
                }
                break;
            }

            // header continuation lines start with WSP
            if line.starts_with(' ') || line.starts_with('\t') {
                if let Some(v) = last_value.as_mut() {
                    v.push_str("\r\n");
                    v.push_str(line.trim_start());
                } else {
                    // malformed; skip
                }
            } else {
                // new header
                if let (Some(n), Some(v)) = (last_name.take(), last_value.take()) {
                    headers.push((n, v));
                }
                // split at first ':'
                if let Some(idx) = line.find(':') {
                    let name = line[..idx].to_string();
                    let value = line[idx + 1..].trim_start().to_string();
                    last_name = Some(name);
                    last_value = Some(value);
                } else {
                    // malformed
                }
            }
        }
    }

    if in_headers {
        // reached EOF while still in headers: finish last header (no body)
        if let (Some(n), Some(v)) = (last_name.take(), last_value.take()) {
            headers.push((n, v));
        }
    }

    let body = body_lines.join("\r\n");
    (headers, body)
}

/// Relaxed header canonicalization (RFC6376 simplified):
/// - Convert header field name to lower-case
/// - Unfold header value (remove CRLF WSP)
/// - Compress WSP to single SP
/// - Remove WSP around ":" (we use the canonical form "name:single-space value")
fn canonicalize_header_relaxed(name: &str, value: &str) -> String {
    let name_l = name.to_lowercase();
    // unfold and compress WSP
    let mut v = value.replace("\r\n", "");
    // replace multiple WSP with single space
    let mut out = String::new();
    let mut prev_wsp = false;
    for ch in v.chars() {
        if ch == ' ' || ch == '\t' {
            if !prev_wsp {
                out.push(' ');
                prev_wsp = true;
            } else {
                // skip extra
            }
        } else {
            out.push(ch);
            prev_wsp = false;
        }
    }
    let val_trimmed = out.trim();
    format!("{}:{}", name_l, format!(" {}", val_trimmed))
}

/// Simple body canonicalization (RFC6376):
/// - no changes except ensure lines end with CRLF
/// - remove trailing empty lines (delete CRLF series at end)
fn canonicalize_body_simple(body: &str) -> String {
    // ensure CRLF line endings
    let mut s = body.replace("\r\n", "\n").replace("\r", "\n");
    // split lines, then rejoin with CRLF
    let mut lines: Vec<&str> = s.split('\n').collect();
    // Remove trailing empty lines
    while let Some(last) = lines.last() {
        if last.is_empty() {
            lines.pop();
        } else {
            break;
        }
    }
    // After removing trailing empty lines, rejoin and add final CRLF
    if lines.is_empty() {
        "".to_string()
    } else {
        let joined = lines.join("\r\n");
        joined + "\r\n"
    }
}

/// Relaxed body canonicalization:
/// - ignore WSP at end of lines
/// - compress WSP within lines to single SP
/// - remove trailing empty lines
fn canonicalize_body_relaxed(body: &str) -> String {
    let s = body.replace("\r\n", "\n").replace("\r", "\n");
    let mut lines: Vec<String> = Vec::new();
    for line in s.split('\n') {
        // trim WSP at end and compress internal WSP
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
        let trimmed = out.trim_end();
        lines.push(trimmed.to_string());
    }
    // remove trailing empty lines
    while let Some(last) = lines.last() {
        if last.is_empty() {
            lines.pop();
        } else {
            break;
        }
    }
    if lines.is_empty() {
        "".to_string()
    } else {
        let joined = lines.join("\r\n");
        joined + "\r\n"
    }
}

/// Compute body hash (bh) base64 for given canonicalization algorithm
fn compute_body_hash_b64(body: &str, body_can: &str) -> String {
    let canonical = match body_can {
        "simple" => canonicalize_body_simple(body),
        "relaxed" => canonicalize_body_relaxed(body),
        _ => canonicalize_body_simple(body),
    };
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let digest = hasher.finalize();
    general_purpose::STANDARD.encode(digest)
}

/// Parse DKIM-Signature header into fields map (very small parser)
fn parse_dkim_signature_header(value: &str) -> Result<DkimSignatureFields, DkimError> {
    // DKIM-Signature: tag1=val1; tag2=val2; ...
    // value might include folding; assume input already unfolded
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

    // Remove leading "DKIM-Signature:" if present
    let mut v = value.trim();
    if v.to_lowercase().starts_with("dkim-signature:") {
        v = v["dkim-signature:".len()..].trim();
    }

    // split by ';' but allow values to contain '='
    for part in v.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(eq) = part.find('=') {
            let k = part[..eq].trim();
            let val = part[eq + 1..].trim();
            match k {
                "v" => fields.v = Some(val.to_string()),
                "a" => fields.a = Some(val.to_string()),
                "c" => fields.c = Some(val.to_string()),
                "d" => fields.d = Some(val.to_string()),
                "s" => fields.s = Some(val.to_string()),
                "h" => fields.h = Some(val.to_string()),
                "bh" => fields.bh = Some(val.to_string()),
                "b" => fields.b = Some(val.to_string()),
                _ => {
                    // ignore unknown tag
                }
            }
        } else {
            // skip
        }
    }
    Ok(fields)
}

/// Extract DKIM-Signature header raw value and index from headers vec (first occurrence)
fn find_dkim_header(headers: &[(String, String)]) -> Option<(usize, String)> {
    for (i, (name, value)) in headers.iter().enumerate() {
        if name.eq_ignore_ascii_case("DKIM-Signature") {
            return Some((i, value.clone()));
        }
    }
    None
}

/// Build the header canonicalization string over headers listed in `h` tag.
/// For DKIM, the signed header block is the concatenation of the canonicalized header fields (in order)
/// For the DKIM-Signature header itself, the 'b' tag must be excluded (i.e., set empty) when signing/verification.
fn build_signed_headers_string(
    headers: &[(String, String)],
    dkim_index: usize,
    h_list: &str,
    header_can: &str,
) -> String {
    // h_list: header names separated by ':'
    // The order in h_list is the order the signer used. For each header name, the last instance (closest to body) is used.
    let want: Vec<&str> = h_list
        .split(':')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    // For quick find, we need to find last occurrence for each header name.
    // headers are in order as they appear (top to bottom). DKIM requires the last matching header to be used.
    let mut out = String::new();
    for name in want.iter() {
        let mut found: Option<(String, String)> = None;
        for (hname, hvalue) in headers.iter() {
            if hname.eq_ignore_ascii_case(name) {
                found = Some((hname.clone(), hvalue.clone()));
            }
        }
        // Special handling: if the header we need is "dkim-signature", use the DKIM header at dkim_index but with b= removed (empty)
        if let Some((hname, hvalue)) = found {
            if hname.eq_ignore_ascii_case("DKIM-Signature") {
                // remove b=... value (everything after 'b=' up to end or semicolon?). For signing, the header's "b=" value is set to empty string.
                // We'll reconstruct by parsing tags and setting b=
                let mut v = hvalue.clone();
                // Replace b=... with b=
                // naive approach: find "b=" and replace following chars (until ';' or end) with empty string
                if let Some(bpos) = v.to_lowercase().find("b=") {
                    // find semicolon after b=
                    let rest = v[bpos..].to_string();
                    if let Some(semi) = rest.find(';') {
                        // keep up to b= and then add ' ' (empty) and keep remainder
                        let before = &v[..bpos + 2]; // include 'b='
                        let after = &v[bpos + 2 + semi..]; // after the semicolon
                        let newv = format!("{}{}", before, after);
                        // set v = newv
                        v = newv;
                    } else {
                        // no semicolon; just truncate after b=
                        let before = &v[..bpos + 2];
                        v = before.to_string();
                    }
                }
                // canonicalize header name/value according to header_can
                if header_can == "relaxed" {
                    out.push_str(&canonicalize_header_relaxed(&hname, &v));
                } else {
                    // simple canonicalization: name exactly, single colon, value as-is then CRLF
                    out.push_str(&format!("{}: {}\r\n", hname, v));
                }
            } else {
                if header_can == "relaxed" {
                    out.push_str(&canonicalize_header_relaxed(&hname, &hvalue));
                } else {
                    out.push_str(&format!("{}: {}\r\n", hname, hvalue));
                }
            }
        } else {
            // header not found; DKIM allows signing headers that are missing? For our test we assume present
        }
    }
    out
}

/// Verify DKIM signature for RSA-SHA256
///
/// Returns Ok(()) on success, Err(DkimError) on failure.
pub fn verify_dkim_signature_rsa(raw_email: &str, pubkey_pem: &str) -> Result<(), DkimError> {
    let (headers, body) = split_headers_body(raw_email);

    // find DKIM header
    let (dkim_idx, dkim_raw) = find_dkim_header(&headers).ok_or(DkimError::NoDkimSignature)?;

    // parse DKIM fields
    let dkim_fields = parse_dkim_signature_header(&dkim_raw).map_err(|e| e)?;

    let bh = dkim_fields.bh.ok_or(DkimError::BadSignatureField)?;
    let b_sig = dkim_fields.b.ok_or(DkimError::BadSignatureField)?;
    let h_list = dkim_fields.h.ok_or(DkimError::BadSignatureField)?;
    // canonicalization: default relaxed/simple if absent
    let c = dkim_fields
        .c
        .unwrap_or_else(|| "relaxed/simple".to_string());
    let parts: Vec<&str> = c.split('/').collect();
    let header_can = parts.get(0).map(|s| *s).unwrap_or("relaxed");
    let body_can = parts.get(1).map(|s| *s).unwrap_or("simple");

    // 1) verify body hash (bh)
    let computed_bh = compute_body_hash_b64(&body, body_can);
    if computed_bh != bh {
        return Err(DkimError::BodyHashMismatch);
    }

    // 2) build signed header block string in canonicalized form
    let signed_headers_str = build_signed_headers_string(&headers, dkim_idx, &h_list, header_can);
    // The DKIM signing input is the canonicalized header block (bytes)
    // compute SHA256 then verify signature using RSA PKCS1v15 with sha256
    // decode signature b (base64)
    let sig_bytes = general_purpose::STANDARD
        .decode(&b_sig)
        .map_err(|e| DkimError::Other(format!("base64 decode sig: {}", e)))?;

    // parse pubkey pem to RsaPublicKey
    // accept PKCS1 or PKCS8? Here we try PKCS1 (BEGIN RSA PUBLIC KEY) or SubjectPublicKeyInfo (BEGIN PUBLIC KEY)
    let pubkey = RsaPublicKey::from_pkcs1_pem(pubkey_pem)
        .or_else(|_| RsaPublicKey::from_public_key_pem(pubkey_pem))
        .map_err(|e| DkimError::PublicKeyParse(format!("{:?}", e)))?;

    // verify signature
    let mut hasher = Sha256::new();
    hasher.update(signed_headers_str.as_bytes());
    let digest = hasher.finalize();

    let padding = PaddingScheme::PKCS1v15Sign {
        hash: Some(rsa::hash::Hash::SHA2_256),
    };
    match pubkey.verify(padding, &digest, &sig_bytes) {
        Ok(_) => Ok(()),
        Err(_) => Err(DkimError::SignatureVerifyFailed),
    }
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
