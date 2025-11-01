use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;
use uuid::Uuid;

const UNSPLASH_AUTH_URL: &'static str = "https://unsplash.com/oauth/authorize";
const UNSPLASH_TOKEN_URL: &'static str = "https://unsplash.com/oauth/token";

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    session_id: Uuid,
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

    let url = Url::parse_with_params(
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

    let _ = webbrowser::open(url.as_str());

    state.sessions.write().await.insert(session_id, None);

    (StatusCode::OK, Json(AuthResponse { session_id }))
}

async fn exchange(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    use reqwest::Client;
    use reqwest::header::{HeaderValue, USER_AGENT};

    let code = params.get("code").unwrap();
    let session_id = Uuid::parse_str(params.get("state").unwrap()).unwrap();

    if let None = state.sessions.read().await.get(&session_id) {
        return StatusCode::NOT_FOUND;
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

    StatusCode::OK
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::routing::{Router, get};
    use tokio::net::TcpListener;

    let _ = dotenvy::dotenv();

    let config = Arc::new(AppState {
        sessions: RwLock::new(HashMap::new()),
        access_key: std::env::var("UNSPLASH_ACCESS_KEY")?,
        secret_key: std::env::var("UNSPLASH_SECRET_KEY")?,
    });

    let app = Router::new()
        .route("/auth", get(auth))
        .route("/exchange", get(exchange))
        .with_state(config);

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
