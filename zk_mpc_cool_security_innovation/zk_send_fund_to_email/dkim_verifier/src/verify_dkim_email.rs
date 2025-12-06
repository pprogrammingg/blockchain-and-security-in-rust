use crate::errors::DkimError;

pub enum DkimAlgorithm {
    RSA,
    ECDSA,
}

/// Verifies DKIM signature (returns true/false)
pub fn verify_dkim_signature(
    raw_email: &str,
    pub_key_pem: &str,
    algorithm: DkimAlgorithm,
) -> Result<bool, DkimError> {
    !unimplemented!()
}

/// Canonicalize header/body (RFC-like minimal implementation)
pub fn canonicalize_email(raw: &str) -> (String /* headers */, String /* body */) {
    !unimplemented!()
}

/// Compute Poseidon hash (returns field element or bytes)
pub fn poseidon_hash_bytes(input: &[u8]) -> Vec<u8> {
    !unimplemented!()
}
