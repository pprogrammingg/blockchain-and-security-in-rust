use crate::utils::{canonicalize_body, canonicalize_headers, extract_body};
use base64::{Engine as _, engine::general_purpose};
use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::signature::{SignatureEncoding, Signer, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Generates a DKIM-Signature header for testing purposes.
///
/// This function is used to create a DKIM-Signature for an email, suitable for
/// testing DKIM verification flows. It does **not** send the email or modify
/// the original message; it only returns the DKIM header as a string.
///
/// # Steps
/// 1. Extract the email body and canonicalize it according to `canon_type`.
/// 2. Compute the SHA-256 body hash (`bh=`) from the canonicalized body.
/// 3. Build the DKIM-Signature prefix (without `b=` signature).
/// 4. Canonicalize the headers specified in `headers_to_sign`.
/// 5. Hash the canonicalized headers and generate an RSA signature.
/// 6. Encode the signature in Base64 and append to the DKIM header.
///
/// # Parameters
/// - `raw_email`: The full raw email text (headers + body).
/// - `priv_key`: The RSA private key to sign the headers.
/// - `selector`: DKIM selector (`s=` in the DKIM header).
/// - `domain`: Signing domain (`d=` in the DKIM header).
/// - `headers_to_sign`: List of headers to include in the signature, in order.
/// - `canon_type`: Canonicalization type, e.g., `"relaxed"` or `"simple"`.
///
/// # Returns
/// - `Ok(String)` containing the full DKIM-Signature header including `b=`.
/// - `Err(String)` if any step fails (body extraction, header canonicalization, signing).
///
/// # Notes
/// - The canonicalization rules must exactly match the verifier’s expectations.
/// - Only minimal DKIM features are implemented; suitable for testing.
pub fn sign_email(
    raw_email: &str,
    priv_key: &RsaPrivateKey,
    selector: &str,
    domain: &str,
    headers_to_sign: &[&str],
    canon_type: &str,
) -> Result<String, String> {
    // Body hash
    let body = extract_body(raw_email)?;
    let canon_body = canonicalize_body(&body, canon_type);
    let bh = general_purpose::STANDARD.encode(Sha256::digest(canon_body.as_bytes()));

    // DKIM header WITHOUT signature
    let dkim_header_without_b = format!(
        "DKIM-Signature: v=1; a=rsa-sha256; c={0}/{0}; d={1}; s={2}; bh={3}; b=",
        canon_type, domain, selector, bh
    );

    // Email with DKIM header included
    let email_with_dkim = format!("{}\r\n{}", dkim_header_without_b, raw_email);

    // Canonicalize headers (including DKIM-Signature)
    let headers_canon = canonicalize_headers(&email_with_dkim, headers_to_sign, canon_type)?;

    // Sign canonicalized headers (NOT a hash!)
    let signing_key = SigningKey::<Sha256>::new(priv_key.clone());
    let sig: Signature = signing_key.sign(headers_canon.as_bytes());

    Ok(format!(
        "{}{}",
        dkim_header_without_b,
        general_purpose::STANDARD.encode(sig.to_bytes())
    ))
}

/// Verifies a DKIM-signed email using the provided RSA public key.
///
/// This function checks that the email body and the specified headers
/// canonicalize to the same values used in the DKIM signature. It ensures
/// both the `bh=` body hash and `b=` header signature are valid.
///
/// # Steps
/// 1. Parse the DKIM header into key-value pairs.
/// 2. Extract `bh=` (body hash) and `b=` (header signature).
/// 3. Canonicalize the email body and compute its SHA-256 hash, comparing
///    to `bh=`.
/// 4. Canonicalize the specified headers and compute the SHA-256 hash.
/// 5. Decode the Base64 signature and verify it against the header hash (RSA verify using
/// public key)
///
/// # Parameters
/// - `raw_email`: Full raw email text (headers + body).
/// - `pub_key`: The RSA public key corresponding to the signing private key.
/// - `dkim_header`: DKIM-Signature header to verify.
/// - `headers_to_verify`: Headers to include in signature verification, in order.
/// - `canon_type`: Canonicalization type, e.g., `"relaxed"` or `"simple"`.
///
/// # Returns
/// - `Ok(())` if both body hash and header signature are valid.
/// - `Err(String)` if verification fails (body hash mismatch, invalid signature, missing fields).
///
/// # Notes
/// - Body hash mismatch often indicates differences in canonicalization or tampering.
/// - Header canonicalization must exactly match the signer’s method.
pub fn verify_email(
    raw_email: &str,
    pub_key: &RsaPublicKey,
    dkim_header: &str,
    headers_to_verify: &[&str],
    canon_type: &str,
) -> Result<(), String> {
    let dkim_map: HashMap<_, _> = dkim_header
        .split(';')
        .filter_map(|part| part.trim().split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let bh = dkim_map.get("bh").ok_or("Missing bh field")?;
    let b = dkim_map.get("b").ok_or("Missing b field")?;

    let body = extract_body(raw_email)?;
    let canon_body = canonicalize_body(&body, canon_type);

    // calc body hash to verify against bh
    if general_purpose::STANDARD.encode(Sha256::digest(canon_body.as_bytes())) != *bh {
        return Err("Body hash mismatch".into());
    }

    // calc header hash to verify as part RSA verification pipeline
    let headers_canon = canonicalize_headers(raw_email, headers_to_verify, canon_type)?;
    let digest = Sha256::digest(headers_canon.as_bytes());

    let sig_bytes = general_purpose::STANDARD
        .decode(b)
        .map_err(|_| "Invalid base64 signature")?;
    let signature = Signature::try_from(sig_bytes.as_slice()).map_err(|_| "Invalid signature")?;

    let verifying_key = VerifyingKey::<Sha256>::new(pub_key.clone());
    verifying_key
        .verify(&digest, &signature)
        .map_err(|_| "Signature verification failed".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_sign_verify() {
        use rand::rngs::OsRng;

        const RAW_EMAIL: &str = "From: alice@example.com\r\n\
         To: bob@example.com\r\n\
         Subject: DKIM Test\r\n\
         Date: Wed, 05 Nov 2025 17:35:02 +0000\r\n\
         \r\n\
         Hello DKIM!";

        let mut rng = OsRng;
        let priv_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let pub_key = RsaPublicKey::from(&priv_key);

        let headers_to_sign = ["from", "to", "subject", "date", "dkim-signature"];

        let canon = "relaxed";

        // Produce DKIM-Signature header
        let dkim_header = sign_email(
            RAW_EMAIL,
            &priv_key,
            "selector",
            "example.com",
            &headers_to_sign,
            canon,
        )
        .unwrap();

        // Verify using raw email + DKIM header
        assert!(verify_email(RAW_EMAIL, &pub_key, &dkim_header, &headers_to_sign, canon).is_ok());

        // Tamper body
        let tampered = RAW_EMAIL.replace("Hello DKIM!", "Hello tampered!");
        assert!(verify_email(&tampered, &pub_key, &dkim_header, &headers_to_sign, canon).is_err());
    }
}
