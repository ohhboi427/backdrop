use axum::response::{IntoResponse, Response};
use http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authentication failed")]
    AuthFailed,
    #[error("Invalid or unexpected response")]
    InvalidResponse,
    #[error("Invalid session identifier")]
    InvalidSession,
    #[error("Missing query parameter")]
    MissingParam,
    #[error("Network error")]
    NetworkError,
    #[error("Token is not ready")]
    TokenNotReady,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::AuthFailed => StatusCode::UNAUTHORIZED,
            Error::InvalidResponse => StatusCode::BAD_REQUEST,
            Error::InvalidSession => StatusCode::UNAUTHORIZED,
            Error::MissingParam => StatusCode::BAD_REQUEST,
            Error::NetworkError => StatusCode::BAD_GATEWAY,
            Error::TokenNotReady => StatusCode::NOT_FOUND,
        }
        .into_response()
    }
}
