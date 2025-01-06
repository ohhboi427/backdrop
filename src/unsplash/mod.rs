use std::collections::HashMap;

use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as HttpClient, RequestBuilder, Response,
};
use serde::{Deserialize, Serialize};

pub mod error;
pub mod result;

pub use error::Error;
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

#[derive(Debug, Clone)]
pub enum Quality {
    Raw,
    Custom(u32, u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    id: String,
    urls: HashMap<String, String>,
    links: HashMap<String, String>,
}

impl Photo {
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone)]
pub struct Client {
    http: HttpClient,
}

impl Client {
    pub fn new<T: AsRef<str>>(api_key: T) -> Result<Self> {
        let auth = format!("Client-ID {}", api_key.as_ref());
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

    pub async fn find_topic<T: AsRef<str>>(&self, id_or_slug: T) -> Result<Topic> {
        let request = self
            .http
            .get(unsplash_api!("/topics/{}", id_or_slug.as_ref()));

        let response = Self::send_request(request).await?;
        let topic = response.json().await.map_err(|_| Error::InvalidResponse)?;

        Ok(topic)
    }

    pub async fn fetch_photos(&self, topic: &Topic, count: u32) -> Result<Vec<Photo>> {
        let request = self
            .http
            .get(unsplash_api!("/photos/random"))
            .query(query_params!(
                "count" => count,
                "topics" => topic.id
            ));

        let response = Self::send_request(request).await?;
        let photos = response.json().await.map_err(|_| Error::InvalidResponse)?;

        Ok(photos)
    }

    pub async fn download_photo(&self, photo: &Photo, quality: Quality) -> Result<Bytes> {
        let track_request = self.http.get(&photo.links["download_location"]);

        Self::send_request(track_request).await?;

        let mut download_request = self.http.get(&photo.urls["raw"]).query(query_params!(
            "fm" => "png",
        ));

        if let Quality::Custom(w, h) = quality {
            download_request = download_request.query(query_params!(
                "w" => w,
                "h" => h,
                "fit" => "min",
            ));
        }

        let response = Self::send_request(download_request).await?;
        let data = response.bytes().await.map_err(|_| Error::InvalidResponse)?;

        Ok(data)
    }

    async fn send_request(request: RequestBuilder) -> Result<Response> {
        let response = request.send().await.map_err(|_| Error::Request)?;

        if !response.status().is_success() {
            return Err(Error::Status(response.status()));
        }

        Ok(response)
    }
}
