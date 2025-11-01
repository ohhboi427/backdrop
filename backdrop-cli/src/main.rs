use reqwest::Client;
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let auth_url = Url::parse("http://localhost:8000/auth")?;

    let client = Client::new();
    client.get(auth_url).send().await?;

    Ok(())
}
