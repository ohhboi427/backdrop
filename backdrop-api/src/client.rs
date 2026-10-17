use crate::Result;
use crate::auth::Auth;
use crate::endpoints::Endpoint;
use http::Method;
use reqwest::header::{self, HeaderMap, HeaderValue};
use url::Url;

pub(crate) const BACKDROP_USER_AGENT: &str = "backdrop/1.0";
const UNSPLASH_API_URL: &str = "https://api.unsplash.com/";

pub struct Client {
    http: reqwest::Client,
}

impl Client {
    pub fn new(auth: Auth) -> Result<Self> {
        let auth_header_str = match auth {
            Auth::ClientId(id) => format!("Client-ID {}", id),
            Auth::Bearer(token) => format!("Bearer {}", token),
        };

        let mut auth_header = HeaderValue::try_from(&auth_header_str).unwrap();
        auth_header.set_sensitive(true);

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header);

        let http = reqwest::Client::builder()
            .user_agent(BACKDROP_USER_AGENT)
            .default_headers(headers)
            .build()?;

        Ok(Self { http })
    }

    pub async fn execute<E: Endpoint>(&self, endpoint: E) -> Result<E::Response> {
        let path = endpoint.path();
        let url = Url::parse(&format!("{UNSPLASH_API_URL}{path}"))?;

        let mut req = self.http.request(E::METHOD, url);
        req = match E::METHOD {
            Method::GET | Method::DELETE => req.query(&endpoint),
            _ => req.json(&endpoint),
        };

        let resp = req.send().await?.error_for_status()?;

        Ok(resp.json().await?)
    }
}
