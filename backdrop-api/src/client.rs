use crate::Result;
use crate::auth::{Auth, BACKDROP_USER_AGENT};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

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
        headers.insert(AUTHORIZATION, auth_header);

        let http = reqwest::Client::builder()
            .user_agent(BACKDROP_USER_AGENT)
            .default_headers(headers)
            .build()?;

        Ok(Self { http })
    }
}
