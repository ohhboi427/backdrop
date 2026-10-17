use crate::LoginError;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Login error: {0}")]
    LoginError(#[from] LoginError),

    #[error("Backdrop error: {0}")]
    BackdropError(#[from] backdrop_api::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
