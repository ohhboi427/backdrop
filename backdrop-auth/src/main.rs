use axum::Json;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use url::Url;
use uuid::Uuid;

const UNSPLASH_AUTH_URL: &'static str = "https://unsplash.com/oauth/authorize";
const UNSPLASH_TOKEN_URL: &'static str = "https://unsplash.com/oauth/token";

#[derive(Debug, Error)]
enum Error {
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

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeResponse {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: u64,
}

#[derive(Debug)]
struct AppState {
    sessions: RwLock<HashMap<Uuid, Option<ExchangeResponse>>>,
    server_url: Url,
    access_key: String,
    secret_key: String,
}

async fn auth(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    use unsplash_api::auth::AuthResponse;

    let session_id = Uuid::new_v4();

    let redirect_url = state.server_url.join("/exchange").unwrap();
    let auth_url = Url::parse_with_params(
        UNSPLASH_AUTH_URL,
        [
            ("client_id", state.access_key.as_str()),
            ("redirect_uri", redirect_url.as_str()),
            ("response_type", "code"),
            ("scope", "public"),
            ("state", session_id.to_string().as_str()),
        ],
    )
    .unwrap();

    state.sessions.write().await.insert(session_id, None);

    Json(AuthResponse {
        session_id,
        auth_url,
    })
}

async fn exchange(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    use http::header::{HeaderValue, USER_AGENT};
    use reqwest::Client;

    let session_id = Uuid::parse_str(params.get("state").ok_or(Error::MissingParam)?)
        .map_err(|_| Error::InvalidResponse)?;

    if let None = state.sessions.read().await.get(&session_id) {
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
    let token_url = Url::parse_with_params(
        UNSPLASH_TOKEN_URL,
        [
            ("client_id", state.access_key.as_str()),
            ("client_secret", state.secret_key.as_str()),
            ("redirect_uri", redirect_url.as_str()),
            ("code", code.as_str()),
            ("grant_type", "authorization_code"),
        ],
    )
    .unwrap();

    let client = Client::new();
    let response = client
        .post(token_url)
        .header(USER_AGENT, HeaderValue::from_static("Backdrop/2.0"))
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
        .insert(session_id, Some(response));

    Ok(StatusCode::OK)
}

async fn token(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    use unsplash_api::auth::AuthToken;

    let session_id = Uuid::parse_str(params.get("state").ok_or(Error::MissingParam)?)
        .map_err(|_| Error::InvalidResponse)?;

    let mut sessions = state.sessions.write().await;
    match sessions.get(&session_id) {
        Some(Some(_)) => {
            let response = sessions.remove(&session_id).unwrap().unwrap();

            Ok(Json(AuthToken::Bearer(response.access_token)))
        }
        Some(None) => Err(Error::TokenNotReady),
        None => Err(Error::InvalidSession),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::routing::{Router, get, post};
    use tokio::net::TcpListener;

    let _ = dotenvy::dotenv();

    let config = Arc::new(AppState {
        sessions: RwLock::new(HashMap::new()),
        server_url: Url::parse(std::env::var("SERVER_URL")?.as_str())?,
        access_key: std::env::var("UNSPLASH_ACCESS_KEY")?,
        secret_key: std::env::var("UNSPLASH_SECRET_KEY")?,
    });

    let app = Router::new()
        .route("/auth", post(auth))
        .route("/exchange", get(exchange))
        .route("/token", get(token))
        .with_state(config);

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
