use clap::{Parser, Subcommand};
use thiserror::Error;
use unsplash_api::auth::AuthToken;

const SERVER_URL: &'static str = "http://localhost:8000/";

#[derive(Debug, Error)]
enum Error {
    #[error("Authentication failed")]
    AuthFailed,
    #[error("Invalid or unexpected response")]
    InvalidResponse,
    #[error("Network error")]
    NetworkError,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Auth,
}

async fn auth() -> Result<AuthToken, Error> {
    use http::StatusCode;
    use http::header::{CONTENT_TYPE, HeaderValue};
    use reqwest::Client;
    use tokio::time::Duration;
    use unsplash_api::auth::AuthResponse;
    use url::Url;

    let server_url = Url::parse(SERVER_URL).unwrap();

    let auth_url = server_url.join("auth").unwrap();

    let client = Client::new();
    let auth_response: AuthResponse = client
        .post(auth_url)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .send()
        .await
        .map_err(|_| Error::NetworkError)?
        .json()
        .await
        .map_err(|_| Error::InvalidResponse)?;

    webbrowser::open(auth_response.auth_url.as_str()).unwrap();

    let mut token_url = server_url.join("token").unwrap();
    token_url
        .query_pairs_mut()
        .append_pair("state", auth_response.session_id.to_string().as_str());

    let token = tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            let response = client
                .get(token_url.as_str())
                .send()
                .await
                .map_err(|_| Error::NetworkError)?;

            let status = response.status();

            if status.is_success() {
                return Ok(response
                    .json::<AuthToken>()
                    .await
                    .map_err(|_| Error::InvalidResponse)?);
            }

            if status == StatusCode::NOT_FOUND {
                continue;
            }

            return Err(Error::AuthFailed);
        }
    })
    .await
    .map_err(|_| Error::AuthFailed)??;

    Ok(token)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use keyring::Entry;
    use unsplash_api::client::Client;

    let token_entry = Entry::new("Backdrop", "bearer_token")?;

    let cli = Cli::parse();
    let token = match cli.command {
        Some(Commands::Auth) => {
            let token = auth().await?;
            token_entry.set_password(token.as_ref())?;

            token
        }
        None => match token_entry.get_password() {
            Ok(token) => AuthToken::Bearer(token),
            Err(keyring::Error::NoEntry) => {
                let token = auth().await?;
                token_entry.set_password(token.as_ref())?;

                token
            }
            Err(e) => return Err(e.into()),
        },
    };

    println!("{}", token);
    let _client = Client::new(&token);

    Ok(())
}
