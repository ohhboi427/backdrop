use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    session_id: Uuid,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let auth_url = Url::parse("http://localhost:8000/auth")?;

    let client = Client::new();
    let auth_response: AuthResponse = client.post(auth_url).send().await?.json().await?;

    let token_url = Url::parse_with_params(
        "http://localhost:8000/token",
        [("state", &auth_response.session_id.to_string())],
    )?;

    sleep(Duration::from_secs(30)).await;

    let token: String = client.get(token_url).send().await?.json().await?;

    println!("{}", token);

    Ok(())
}
