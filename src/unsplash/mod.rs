use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as HttpClient, RequestBuilder, Response,
};
use serde::{Deserialize, Serialize};

mod models;
pub use models::Photo;
use models::Topic;

mod error;
pub use error::Error;

mod result;
pub use result::Result;

macro_rules! unsplash_api {
    ($end_point:expr) => {
        concat!("https://api.unsplash.com", $end_point)
    };

    ($end_point:expr, $($arg:expr),+) => {
        format!(unsplash_api!($end_point), $($arg),+)
    };
}

macro_rules! query_params {
    ($($key:expr => $value:expr),+ $(,)?) => {
        &[
            $(($key, $value.to_string())),+
        ]
    };
}

type QueryParam = (&'static str, String);

trait QueryParams {
    fn query_params(&self) -> Vec<QueryParam>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "category", content = "value", rename_all = "snake_case")]
pub enum Query {
    Text(String),
    Topic(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fetch {
    count: u32,
    query: Option<Query>,
}

impl Default for Fetch {
    fn default() -> Self {
        Self {
            count: 1,
            query: None,
        }
    }
}

impl QueryParams for Fetch {
    fn query_params(&self) -> Vec<QueryParam> {
        let params = Vec::from(query_params!(
            "count" => self.count,
            "orientation" => "landscape",
        ));

        params
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Format {
    Png,
    Jpeg { quality: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum Resolution {
    Raw,
    Custom { width: u32, height: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    format: Format,
    resolution: Resolution,
}

impl Default for Download {
    fn default() -> Self {
        Self {
            format: Format::Png,
            resolution: Resolution::Custom {
                width: 1920,
                height: 1080,
            },
        }
    }
}

impl QueryParams for Download {
    fn query_params(&self) -> Vec<QueryParam> {
        let mut params = Vec::from(query_params!(
            "fm" => "png",
        ));

        if let Resolution::Custom { width, height } = self.resolution {
            params.extend_from_slice(query_params!(
                "w" => width,
                "h" => height,
                "fit" => "min",
            ))
        }

        params
    }
}

#[derive(Clone)]
pub struct Client {
    http: HttpClient,
}

impl Client {
    pub fn new(api_key: &str) -> Result<Self> {
        let auth = format!("Client-ID {}", api_key);
        let mut auth = HeaderValue::from_str(&auth).map_err(|_| Error::InvalidApiKey)?;
        auth.set_sensitive(true);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", auth);

        Ok(Self {
            http: HttpClient::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        })
    }

    pub async fn fetch_photos(&self, fetch: &Fetch) -> Result<Vec<Photo>> {
        let mut request = self
            .http
            .get(unsplash_api!("/photos/random"))
            .query(&fetch.query_params());

        if let Some(query) = &fetch.query {
            match query {
                Query::Text(text) => {
                    request = request.query(query_params!(
                        "query" => text
                    ))
                }

                Query::Topic(id_or_slug) => {
                    let topic = self.find_topic(&id_or_slug).await?;
                    request = request.query(query_params!(
                        "topics" => topic.id()
                    ));
                }
            }
        }

        let response = Self::send_request(request).await?;
        let photos = response.json().await.map_err(|_| Error::InvalidResponse)?;

        Ok(photos)
    }

    pub async fn download_photo(&self, photo: &Photo, download: &Download) -> Result<Bytes> {
        let track_request = self.http.get(photo.download_track_url());
        Self::send_request(track_request).await?;

        let download_request = self
            .http
            .get(photo.file_url())
            .query(&download.query_params());

        let response = Self::send_request(download_request).await?;
        let data = response.bytes().await.map_err(|_| Error::InvalidResponse)?;

        Ok(data)
    }

    async fn find_topic(&self, id_or_slug: &str) -> Result<Topic> {
        let request = self.http.get(unsplash_api!("/topics/{}", id_or_slug));

        let response = Self::send_request(request).await?;
        let topic = response.json().await.map_err(|_| Error::InvalidResponse)?;

        Ok(topic)
    }

    async fn send_request(request: RequestBuilder) -> Result<Response> {
        let response = request.send().await.map_err(|_| Error::Request)?;

        if !response.status().is_success() {
            return Err(Error::Status(response.status()));
        }

        Ok(response)
    }
}
