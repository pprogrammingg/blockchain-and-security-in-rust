use thiserror::Error;

#[derive(Debug, Error)]
pub enum DkimError {
    #[error("{0}")]
    DkimKeyFetch(String),

    #[error("{0}")]
    DkimVerifier(String),
}

pub trait ToDkimError<T> {
    fn to_dkim_key_fetch_err(self) -> Result<T, DkimError>;
    fn to_dkim_verifier_err(self) -> Result<T, DkimError>;
}

impl<T, E: std::fmt::Display> ToDkimError<T> for Result<T, E> {
    fn to_dkim_key_fetch_err(self) -> Result<T, DkimError> {
        self.map_err(|e| DkimError::DkimKeyFetch(e.to_string()))
    }

    fn to_dkim_verifier_err(self) -> Result<T, DkimError> {
        self.map_err(|e| DkimError::DkimVerifier(e.to_string()))
    }
}
