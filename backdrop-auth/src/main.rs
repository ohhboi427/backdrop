mod error;
mod routes;
mod state;

use crate::state::AppState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;

async fn purge(state: Arc<AppState>) {
    use std::time::Duration;

    let mut ticker = tokio::time::interval(Duration::from_secs(60));

    loop {
        state
            .sessions
            .write()
            .await
            .retain(|_, (_, created_at)| created_at.elapsed() < Duration::from_secs(300));

        ticker.tick().await;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::routing::{Router, get, post};
    use tokio::net::TcpListener;

    use crate::routes::*;

    let _ = dotenvy::dotenv();

    let config = Arc::new(AppState {
        sessions: RwLock::new(HashMap::new()),
        server_url: Url::parse(std::env::var("SERVER_URL")?.as_str())?,
        access_key: std::env::var("UNSPLASH_ACCESS_KEY")?,
        secret_key: std::env::var("UNSPLASH_SECRET_KEY")?,
    });

    tokio::spawn(purge(Arc::clone(&config)));

    let app = Router::new()
        .route("/auth", post(auth))
        .route("/exchange", get(exchange))
        .route("/token", get(token))
        .with_state(config);

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
