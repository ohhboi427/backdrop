use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authentication failed")]
    AuthFailed,
    #[error("Invalid or unexpected response")]
    InvalidResponse,
    #[error("Network error")]
    NetworkError,
}
