use std::collections::HashMap;

use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as HttpClient, Response,
};
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;

pub mod error;
pub mod result;

use crate::unsplash::Quality::Custom;
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
        let response = self
            .http
            .get(unsplash_api!("/topics/{}", id_or_slug.as_ref()))
            .send()
            .await
            .map_err(|_| Error::Request)?;

        let response = Self::handle_response(response)?;

        Ok(response.json().await.map_err(|_| Error::InvalidResponse)?)
    }

    pub async fn fetch_photos(&self, topic: &Topic, count: u32) -> Result<Vec<Photo>> {
        let response = self
            .http
            .get(unsplash_api!("/photos/random"))
            .query(query_params!(
                "count" => count,
                "topics" => topic.id
            ))
            .send()
            .await
            .map_err(|_| Error::Request)?;

        let response = Self::handle_response(response)?;

        Ok(response.json().await.map_err(|_| Error::InvalidResponse)?)
    }

    pub async fn download_photos(
        &self,
        photos: Vec<Photo>,
        quality: Quality,
    ) -> Vec<(Photo, Result<Bytes>)> {
        let mut tasks = JoinSet::<Result<Bytes>>::new();

        for photo in photos.iter().cloned() {
            let client = self.http.clone();
            let quality = quality.clone();

            tasks.spawn(async move {
                client
                    .get(&photo.links["download_location"])
                    .send()
                    .await
                    .map_err(|_| Error::Request)?;

                let mut request = client.get(&photo.urls["raw"]).query(query_params!(
                    "fm" => "png",
                ));

                if let Custom(w, h) = quality {
                    request = request.query(query_params!(
                        "w" => w,
                        "h" => h,
                        "fit" => "min",
                    ));
                }

                let response = request.send().await.map_err(|_| Error::Request)?;

                let response = Self::handle_response(response)?;

                Ok(response.bytes().await.map_err(|_| Error::InvalidResponse)?)
            });
        }

        photos
            .into_iter()
            .zip(tasks.join_all().await.into_iter())
            .collect()
    }

    fn handle_response(response: Response) -> Result<Response> {
        if !response.status().is_success() {
            return Err(Error::Status(response.status()));
        }

        Ok(response)
    }
}
