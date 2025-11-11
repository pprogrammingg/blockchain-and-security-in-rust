use thiserror::Error;

#[derive(Debug, Error)]
pub enum DkimError {
    #[error("encountered dkim_key_fetch error: {0}")]
    Generic(String),
}

pub trait ToDkimKeyFetchErr<T> {
    fn to_dkim_key_fetch_err(self) -> Result<T, DkimError>;
}

impl<T, E: std::fmt::Display> ToDkimKeyFetchErr<T> for Result<T, E> {
    fn to_dkim_key_fetch_err(self) -> Result<T, DkimError> {
        self.map_err(|e| DkimError::Generic(e.to_string()))
    }
}
