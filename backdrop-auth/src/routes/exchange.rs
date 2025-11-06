use crate::error::Error;
use crate::state::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

const UNSPLASH_TOKEN_URL: &'static str = "https://unsplash.com/oauth/token";

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

pub async fn exchange(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    use http::StatusCode;
    use http::header::{HeaderValue, USER_AGENT};
    use reqwest::Client;
    use unsplash_api::client::USER_AGENT as BACKDROP_USER_AGENT;
    use uuid::Uuid;

    let session_id = Uuid::parse_str(params.get("state").ok_or(Error::MissingParam)?)
        .map_err(|_| Error::InvalidResponse)?;

    if !state.sessions.read().await.contains_key(&session_id) {
        return Err(Error::InvalidSession);
    }

    let code = match params.get("code").ok_or(Error::AuthFailed) {
        Ok(code) => code,
        Err(err) => {
            state.sessions.write().await.remove(&session_id);

            return Err(err);
        }
    };

    let redirect_url = state.server_url.join("/exchange").unwrap();

    let client = Client::new();
    let response: ExchangeResponse = client
        .post(UNSPLASH_TOKEN_URL)
        .header(USER_AGENT, HeaderValue::from_static(BACKDROP_USER_AGENT))
        .form(&[
            ("client_id", state.access_key.as_str()),
            ("client_secret", state.secret_key.as_str()),
            ("redirect_uri", redirect_url.as_str()),
            ("code", code.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|_| Error::NetworkError)?
        .json()
        .await
        .map_err(|_| Error::InvalidResponse)?;

    state
        .sessions
        .write()
        .await
        .entry(session_id)
        .and_modify(|(session, _)| {
            session.replace(response);
        });

    Ok(StatusCode::OK)
}
