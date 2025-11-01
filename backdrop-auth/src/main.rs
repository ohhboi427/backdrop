use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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
enum AuthError {
    #[error("Invalid session identifier")]
    InvalidSession,
    #[error("Missing query parameter")]
    MissingParam,
    #[error("Token is not ready")]
    TokenNotReady,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::InvalidSession => StatusCode::UNAUTHORIZED,
            AuthError::MissingParam => StatusCode::BAD_REQUEST,
            AuthError::TokenNotReady => StatusCode::NOT_FOUND,
        }
        .into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    session_id: Uuid,
    auth_url: Url,
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
    sessions: RwLock<HashMap<Uuid, Option<String>>>,
    access_key: String,
    secret_key: String,
}

async fn auth(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let session_id = Uuid::new_v4();
    let auth_url = Url::parse_with_params(
        UNSPLASH_AUTH_URL,
        [
            ("client_id", state.access_key.as_str()),
            ("redirect_uri", "http://localhost:8000/exchange"),
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
) -> Result<impl IntoResponse, AuthError> {
    use reqwest::Client;
    use reqwest::header::{HeaderValue, USER_AGENT};

    let code = params.get("code").ok_or(AuthError::MissingParam)?;
    let session_id = Uuid::parse_str(params.get("state").ok_or(AuthError::MissingParam)?)
        .map_err(|_| AuthError::MissingParam)?;

    if let None = state.sessions.read().await.get(&session_id) {
        return Err(AuthError::InvalidSession);
    }

    let url = Url::parse_with_params(
        UNSPLASH_TOKEN_URL,
        [
            ("client_id", state.access_key.as_str()),
            ("client_secret", state.secret_key.as_str()),
            ("redirect_uri", "http://localhost:8000/exchange"),
            ("code", code.as_str()),
            ("grant_type", "authorization_code"),
        ],
    )
    .unwrap();

    let client = Client::new();
    let response: ExchangeResponse = client
        .post(url)
        .header(USER_AGENT, HeaderValue::from_str("Backdrop/2.0").unwrap())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    state
        .sessions
        .write()
        .await
        .insert(session_id, Some(response.access_token));

    Ok(StatusCode::OK)
}

async fn token(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AuthError> {
    let session_id = Uuid::parse_str(params.get("state").unwrap()).unwrap();

    let mut sessions = state.sessions.write().await;
    match sessions.get(&session_id) {
        Some(Some(_)) => {
            let token = sessions.remove(&session_id).unwrap().unwrap();

            Ok(Json(token))
        }
        Some(None) => Err(AuthError::TokenNotReady),
        None => Err(AuthError::InvalidSession),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::routing::{Router, get, post};
    use tokio::net::TcpListener;

    let _ = dotenvy::dotenv();

    let config = Arc::new(AppState {
        sessions: RwLock::new(HashMap::new()),
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
