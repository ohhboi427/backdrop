use crate::auth::AuthError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),

    #[error("Network error: {0}")]
    HttpError(#[from] reqwest::Error),
}
