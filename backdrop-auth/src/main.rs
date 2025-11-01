use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;

const UNSPLASH_AUTH_URL: &'static str = "https://unsplash.com/oauth/authorize";
const UNSPLASH_TOKEN_URL: &'static str = "https://unsplash.com/oauth/token";

#[derive(Debug)]
struct Config {
    access_key: String,
    secret_key: String,
}

async fn auth(State(state): State<Arc<Config>>) -> impl IntoResponse {
    let url = Url::parse_with_params(
        UNSPLASH_AUTH_URL,
        [
            ("client_id", state.access_key.as_str()),
            ("redirect_uri", "http://localhost:8000/callback"),
            ("response_type", "code"),
            ("scope", "public"),
        ],
    )
    .unwrap();

    let _ = webbrowser::open(url.as_str());

    StatusCode::OK
}

async fn callback(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<Config>>,
) -> impl IntoResponse {
    let code = params.get("code").unwrap();

    println!("code: {}", code);

    StatusCode::OK
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::routing::{Router, get};
    use tokio::net::TcpListener;

    let _ = dotenvy::dotenv();

    let config = Arc::new(Config {
        access_key: std::env::var("UNSPLASH_ACCESS_KEY")?,
        secret_key: std::env::var("UNSPLASH_SECRET_KEY")?,
    });

    let app = Router::new()
        .route("/auth", get(auth))
        .route("/callback", get(callback))
        .with_state(config);

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
