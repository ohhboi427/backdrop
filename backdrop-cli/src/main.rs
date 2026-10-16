use backdrop_api::auth::{self, TokenResponse};
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use url::Url;

#[derive(Debug, Error)]
enum LoginError {
    #[error("Failed to start local redirect listener on 127.0.0.1")]
    PortConflict,

    #[error("Failed to launch a browser")]
    Browser,

    #[error("OAuth callback received without authorization code")]
    AccessDenied,

    #[error("General network error")]
    NetworkError(#[from] reqwest::Error),

    #[error("General I/O error")]
    IoError(#[from] std::io::Error),
}

async fn capture_auth_code(callback: &TcpListener) -> Result<String, LoginError> {
    let (mut socket, _addr) = callback
        .accept()
        .await
        .map_err(|_| LoginError::PortConflict)?;

    let mut bytes = [0u8; 4096];
    let bytes_read = socket.read(&mut bytes[..]).await?;
    let request_str = String::from_utf8_lossy(&bytes[..bytes_read]);

    let path_and_query = request_str
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or(LoginError::AccessDenied)?;

    let query = path_and_query
        .split_once("?")
        .map(|(_, q)| q)
        .unwrap_or_default();

    let code = form_urlencoded::parse(query.as_bytes())
        .find(|(k, _)| k == "code")
        .map(|(_, v)| v.into_owned())
        .ok_or(LoginError::AccessDenied)?;

    Ok(code)
}

async fn login() -> Result<TokenResponse, LoginError> {
    let callback = TcpListener::bind("127.0.0.1:0").await?;
    let port = callback.local_addr()?.port();
    let redirect_uri = Url::parse(&format!("http://127.0.0.1:{port}/callback")).unwrap();

    let auth_url = auth::build_auth_url(redirect_uri.as_str(), None);
    open::that(auth_url.as_str()).map_err(|_| LoginError::Browser)?;

    let code = capture_auth_code(&callback).await?;
    let token = auth::exchange_token(&code, redirect_uri.as_str()).await?;

    Ok(token)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = login().await?;
    println!("{:#?}", token);

    Ok(())
}
