use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authentication failed")]
    AuthFailed,
    #[error("Failed to interact with token storage")]
    TokenStorageFailed,
    #[error("Invalid or unexpected response")]
    InvalidResponse,
    #[error("Network error")]
    NetworkError,
}
