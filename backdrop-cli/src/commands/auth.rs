use crate::error::Error;
use clap::Subcommand;
use keyring::Entry;
use unsplash_api::auth::AuthToken;

const SERVER_URL: &'static str = "http://localhost:8000/";
const KEY_SERVICE: &'static str = "Backdrop";
const KEY_NAME: &'static str = "bearer_token";

#[derive(Subcommand)]
pub enum AuthCommand {
    Login,
    Logout,
}

impl AuthCommand {
    pub async fn run(self) -> Result<(), Error> {
        match self {
            AuthCommand::Login => {
                obtain_and_store_auth_token().await?;
            }

            AuthCommand::Logout => {
                delete_stored_auth_token()?;
            }
        }

        Ok(())
    }
}

impl Default for AuthCommand {
    fn default() -> Self {
        AuthCommand::Login
    }
}

pub async fn obtain_auth_token() -> Result<AuthToken, Error> {
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

pub async fn obtain_and_store_auth_token() -> Result<AuthToken, Error> {
    let token = obtain_auth_token().await?;

    let entry = Entry::new(KEY_SERVICE, KEY_NAME).unwrap();
    entry
        .set_password(token.as_ref())
        .map_err(|_| Error::TokenStorageFailed)?;

    Ok(token)
}

pub fn get_stored_auth_token() -> Result<Option<AuthToken>, Error> {
    let entry = Entry::new(KEY_SERVICE, KEY_NAME).unwrap();
    match entry.get_password() {
        Ok(token) => Ok(Some(AuthToken::Bearer(token))),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(_) => Err(Error::TokenStorageFailed),
    }
}

pub async fn ensure_auth_token() -> Result<AuthToken, Error> {
    match get_stored_auth_token()? {
        Some(token) => Ok(token),
        None => obtain_and_store_auth_token().await,
    }
}

pub fn delete_stored_auth_token() -> Result<(), Error> {
    let entry = Entry::new(KEY_SERVICE, KEY_NAME).unwrap();
    entry
        .delete_credential()
        .map_err(|_| Error::TokenStorageFailed)?;

    Ok(())
}
