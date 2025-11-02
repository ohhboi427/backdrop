#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use http::StatusCode;
    use reqwest::Client;
    use std::time::Duration;
    use unsplash_api::auth::{AuthResponse, AuthToken};
    use url::Url;

    let auth_url = Url::parse("http://localhost:8000/auth")?;

    let client = Client::new();
    let auth_response: AuthResponse = client.post(auth_url).send().await?.json().await?;

    webbrowser::open(auth_response.auth_url.as_str())?;

    let token_url = Url::parse_with_params(
        "http://localhost:8000/token",
        [("state", &auth_response.session_id.to_string())],
    )?;

    let token = tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            let response = client.get(token_url.as_str()).send().await.unwrap();
            let status = response.status();

            if status.is_success() {
                return Some(response.json::<AuthToken>().await.unwrap());
            }

            if status == StatusCode::NOT_FOUND {
                continue;
            }

            return None;
        }
    })
    .await?;

    println!("{:?}", token);

    Ok(())
}
