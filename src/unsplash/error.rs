use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing or invalid access key")]
    InvalidApiKey,

    #[error("Failed to parse response")]
    InvalidResponse,

    #[error("Failed to send request")]
    Request,

    #[error("HTTP status: {0}")]
    Status(StatusCode),
}

pub type Result<T> = core::result::Result<T, Error>;
