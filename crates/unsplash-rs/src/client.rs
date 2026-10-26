use crate::Result;
use crate::auth::Auth;
use crate::endpoints::Endpoint;
use http::Method;
use reqwest::header::{self, HeaderMap, HeaderValue};
use url::Url;

pub(crate) const DEFAULT_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

const UNSPLASH_API_URL: &str = "https://api.unsplash.com/";

pub struct Client {
    http: reqwest::Client,
    base_url: Url,
}

impl Client {
    pub fn builder(auth: Auth) -> ClientBuilder {
        ClientBuilder::new(auth)
    }

    pub async fn execute<E: Endpoint>(&self, endpoint: E) -> Result<E::Response> {
        let url = self.base_url.join(&endpoint.path())?;

        let mut req = self.http.request(E::METHOD, url);
        req = match E::METHOD {
            Method::GET | Method::DELETE => req.query(&endpoint),
            _ => req.json(&endpoint),
        };

        let resp = req.send().await?.error_for_status()?;

        Ok(resp.json().await?)
    }
}

pub struct ClientBuilder {
    auth: Auth,
    user_agent: Option<String>,
}

impl ClientBuilder {
    pub fn new(auth: Auth) -> Self {
        Self {
            auth,
            user_agent: None,
        }
    }

    pub fn app_info(mut self, name: &str, version: &str) -> Self {
        self.user_agent = Some(format!("{}/{}", name, version));
        self
    }

    pub fn build(self) -> Result<Client> {
        let auth_header_str = match self.auth {
            Auth::ClientId(id) => format!("Client-ID {}", id),
            Auth::Bearer(token) => format!("Bearer {}", token),
        };

        let mut auth_header = HeaderValue::try_from(&auth_header_str).unwrap();
        auth_header.set_sensitive(true);

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header);

        let user_agent = self.user_agent.unwrap_or(DEFAULT_USER_AGENT.into());
        let http = reqwest::Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()?;

        Ok(Client {
            http,
            base_url: Url::parse(UNSPLASH_API_URL)?,
        })
    }
}
