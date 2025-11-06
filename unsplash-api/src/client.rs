use crate::auth::AuthToken;
use reqwest::Client as HttpClient;

pub const USER_AGENT: &'static str = concat!("Backdrop/", env!("CARGO_PKG_VERSION"));

pub struct Client {
    client: HttpClient,
}

impl Client {
    pub fn new(token: &AuthToken) -> Self {
        use self::USER_AGENT as BACKDROP_USER_AGENT;
        use http::header::{AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT};

        let mut auth_header = HeaderValue::from_str(token.to_string().as_str()).unwrap();
        auth_header.set_sensitive(true);

        let headers = HeaderMap::from_iter([
            (AUTHORIZATION, auth_header),
            (USER_AGENT, HeaderValue::from_static(BACKDROP_USER_AGENT)),
        ]);

        let client = HttpClient::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client }
    }
}
