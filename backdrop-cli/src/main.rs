mod error;

use backdrop_api::auth::{self, Auth, TokenResponse};
use backdrop_api::client::Client;
use backdrop_api::endpoints;
pub use error::{Error, Result};
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use url::Url;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Failed to start local redirect listener on 127.0.0.1")]
    PortConflict,

    #[error("Failed to launch a browser")]
    BrowserLaunch,

    #[error("OAuth callback received without authorization code")]
    AccessDenied,
}

async fn capture_auth_code(callback: &TcpListener) -> Result<String> {
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

async fn login() -> Result<TokenResponse> {
    let callback = TcpListener::bind("127.0.0.1:0").await?;
    let port = callback.local_addr()?.port();
    let redirect_uri = Url::parse(&format!("http://127.0.0.1:{port}/callback")).unwrap();

    let auth_url = auth::build_auth_url(&redirect_uri, "public read_user write_user", None);
    open::that(auth_url.as_str()).map_err(|_| LoginError::BrowserLaunch)?;

    let code = capture_auth_code(&callback).await?;
    let token = auth::exchange_token(&code, &redirect_uri).await?;

    Ok(token)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let entry = keyring::Entry::new("backdrop", "default")?;
    let token = match entry.get_password() {
        Ok(token) => Auth::Bearer(token),

        Err(keyring::Error::NoEntry) => {
            let token = login().await?;
            entry.set_password(&token.access_token)?;

            Auth::Bearer(token.access_token)
        }

        Err(e) => return Err(e.into()),
    };

    let client = Client::new(token)?;

    let mut req = endpoints::photo::Random::default();
    req.count = Some(1);

    let photos = client.execute(req).await?;

    println!("{:#?}", photos);

    Ok(())
}
