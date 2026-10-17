use crate::Result;
use crate::client::BACKDROP_USER_AGENT;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

const BACKDROP_AUTH_URI: &str = "https://backdrop-auth-proxy.ohhboi427.workers.dev/exchange";

const UNSPLASH_CLIENT_ID: &str = "s6ac0WqSKlmOLHtY-hlMKzMhywE4vw3aXT9Q9gHzyg8";
const UNSPLASH_AUTH_URI: &str = "https://unsplash.com/oauth/authorize";

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Authorization code is invalid or expired")]
    InvalidCode,

    #[error("OAuth state mismatch")]
    StateMismatch,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Auth {
    ClientId(String),
    Bearer(String),
}

#[derive(Debug, Serialize)]
struct AuthRequest<'a> {
    client_id: &'a str,
    redirect_uri: &'a str,
    response_type: &'a str,
    scope: &'a str,
}

#[derive(Debug, Serialize)]
struct TokenRequest<'a> {
    redirect_uri: &'a str,
    code: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub created_at: u64,
}

pub fn build_auth_url(redirect_uri: &str, scope: &str, _state: Option<&str>) -> Url {
    let auth_params: AuthRequest = AuthRequest {
        client_id: UNSPLASH_CLIENT_ID,
        redirect_uri,
        response_type: "code",
        scope,
    };

    let mut url = Url::parse(UNSPLASH_AUTH_URI).unwrap();
    url.set_query(Some(&serde_urlencoded::to_string(&auth_params).unwrap()));

    url
}

pub async fn exchange_token(code: &str, redirect_uri: &str) -> Result<TokenResponse> {
    let token_params = TokenRequest { redirect_uri, code };

    let response = reqwest::Client::builder()
        .user_agent(BACKDROP_USER_AGENT)
        .build()?
        .post(BACKDROP_AUTH_URI)
        .json(&token_params)
        .send()
        .await?;

    let status = response.status();
    if status == StatusCode::BAD_REQUEST || status == StatusCode::UNAUTHORIZED {
        return Err(AuthError::InvalidCode.into());
    }

    Ok(response.error_for_status()?.json().await?)
}
