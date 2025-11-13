use crate::auth::AuthToken;
use http::Method;
use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use url::Url;

pub const UNSPLASH_API_URL: &'static str = "https://api.unsplash.com/";
pub const USER_AGENT: &'static str = concat!("Backdrop/", env!("CARGO_PKG_VERSION"));

pub struct Client {
    client: HttpClient,
    url: Url,
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

        let url = Url::parse(UNSPLASH_API_URL).unwrap();

        Self { client, url }
    }

    pub(crate) async fn execute<T: DeserializeOwned, S: AsRef<str>>(
        &self,
        method: Method,
        endpoint: S,
    ) -> T {
        let url = self.url.join(endpoint.as_ref()).unwrap();

        let request = self.client.request(method, url).build().unwrap();

        self.client
            .execute(request)
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}
