use crate::Result;
use crate::client::DEFAULT_USER_AGENT;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

const UNSPLASH_AUTH_URL: &str = "https://unsplash.com/oauth/authorize";

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
    redirect_uri: &'a Url,
    response_type: &'a str,
    scope: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct TokenRequest<'a> {
    redirect_uri: &'a Url,
    code: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub created_at: u64,
}

pub struct OAuthConfig {
    pub client_id: String,
    pub redirect_uri: Url,
    pub auth_uri: Url,
    pub proxy_uri: Url,
}

impl OAuthConfig {
    pub fn with_proxy(client_id: impl Into<String>, redirect_uri: Url, proxy_uri: Url) -> Self {
        Self {
            client_id: client_id.into(),
            redirect_uri,
            auth_uri: Url::parse(UNSPLASH_AUTH_URL).unwrap(),
            proxy_uri,
        }
    }

    pub fn build_auth_url(&self, scope: &str, state: Option<&str>) -> Url {
        let auth_params: AuthRequest = AuthRequest {
            client_id: &self.client_id,
            redirect_uri: &self.redirect_uri,
            response_type: "code",
            scope,
            state,
        };

        let mut url = self.auth_uri.clone();
        url.set_query(Some(&serde_urlencoded::to_string(&auth_params).unwrap()));

        url
    }

    pub async fn exchange_token(self, code: &str) -> Result<TokenResponse> {
        let token_params = TokenRequest {
            redirect_uri: &self.redirect_uri,
            code,
        };

        let response = reqwest::Client::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .build()?
            .post(self.proxy_uri)
            .json(&token_params)
            .send()
            .await?;

        let status = response.status();
        if status == StatusCode::BAD_REQUEST || status == StatusCode::UNAUTHORIZED {
            return Err(AuthError::InvalidCode.into());
        }

        Ok(response.error_for_status()?.json().await?)
    }
}
