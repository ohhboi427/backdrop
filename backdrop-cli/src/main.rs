use backdrop_api::auth::*;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use url::Url;

const REDIRECT_BASE: &str = "http://127.0.0.1:8080";
const REDIRECT_URI: &str = "http://127.0.0.1:8080/callback";

#[derive(Debug, Error)]
enum LoginError {
    #[error("Failed to start local redirect listener on 127.0.0.1:8080")]
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

fn extract_code(req: &str) -> Result<String, LoginError> {
    let path_and_query = req
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or(LoginError::AccessDenied)?;

    let url = Url::parse(&format!("{REDIRECT_BASE}{path_and_query}"));
    url.map_err(|_| LoginError::AccessDenied)?
        .query_pairs()
        .find(|(k, _)| k == "code")
        .map(|(_, v)| v.into_owned())
        .ok_or(LoginError::AccessDenied)
}

async fn login() -> Result<TokenResponse, LoginError> {
    let redirect_server = TcpListener::bind("127.0.0.1:8080").await?;

    const AUTH_PARAMS: AuthRequest = AuthRequest {
        client_id: UNSPLASH_CLIENT_ID,
        redirect_uri: REDIRECT_URI,
        response_type: "code",
        scope: "public",
    };

    let mut auth_url = Url::parse(UNSPLASH_AUTH_URI).unwrap();
    auth_url.set_query(Some(&serde_urlencoded::to_string(&AUTH_PARAMS).unwrap()));

    open::that(auth_url.as_str()).map_err(|_| LoginError::Browser)?;

    let (mut socket, _addr) = redirect_server
        .accept()
        .await
        .map_err(|_| LoginError::PortConflict)?;

    let mut bytes = [0u8; 4096];
    let bytes_read = socket.read(&mut bytes[..]).await?;
    let request_str = String::from_utf8_lossy(&bytes[..bytes_read]);

    let code = extract_code(&request_str)?;

    let token_params = TokenRequest {
        redirect_uri: REDIRECT_URI,
        code: &code,
    };

    Ok(reqwest::Client::builder()
        .user_agent(BACKDROP_USER_AGENT)
        .build()?
        .post(BACKDROP_AUTH_URI)
        .json(&token_params)
        .send()
        .await?
        .error_for_status()?
        .json::<TokenResponse>()
        .await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = login().await?;
    println!("{:#?}", token);

    Ok(())
}
