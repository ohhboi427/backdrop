use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::task::JoinSet;

use std::{collections::HashMap, env, fs::File, io::Write};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Topic {
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Photo {
    id: String,
    urls: HashMap<String, String>,
    links: HashMap<String, String>,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Missing or invalid access key")]
    InvalidApiKey,

    #[error("Failed to parse response")]
    InvalidResponse,

    #[error("Failed to send request")]
    Request,

    #[error("HTTP error {0}")]
    Status(StatusCode),
}

type Result<T> = std::result::Result<T, Error>;

fn handle_response(response: Response) -> Result<Response> {
    if !response.status().is_success() {
        return Err(Error::Status(response.status()));
    }

    Ok(response)
}

async fn find_topic<T: AsRef<str>>(client: &Client, id_or_slug: T) -> Result<Topic> {
    let response = client
        .get(unsplash_api!("/topics/{}", id_or_slug.as_ref()))
        .send()
        .await
        .map_err(|_| Error::Request)?;

    let response = handle_response(response)?;

    Ok(response.json().await.map_err(|_| Error::InvalidResponse)?)
}

async fn fetch_photos(client: &Client, topic: &Topic) -> Result<Vec<Photo>> {
    let response = client
        .get(unsplash_api!("/photos/random"))
        .query(query_params!(
            "count" => 10,
            "topics" => topic.id
        ))
        .send()
        .await
        .map_err(|_| Error::Request)?;

    let response = handle_response(response)?;

    Ok(response.json().await.map_err(|_| Error::InvalidResponse)?)
}

async fn download_photos(client: &Client, photos: &[Photo]) -> Vec<Result<Bytes>> {
    let mut tasks = JoinSet::<Result<Bytes>>::new();

    for photo in photos.iter().cloned() {
        let client = client.clone();

        tasks.spawn(async move {
            client
                .get(&photo.links["download_location"])
                .send()
                .await
                .map_err(|_| Error::Request)?;

            let response = client
                .get(&photo.urls["raw"])
                .query(query_params!(
                    "fm" => "png",
                    "w" => 1920,
                    "h" => 1080,
                    "fit" => "min",
                ))
                .send()
                .await
                .map_err(|_| Error::Request)?;

            let response = handle_response(response)?;

            Ok(response.bytes().await.map_err(|_| Error::InvalidResponse)?)
        });
    }

    tasks.join_all().await
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();

    let api_key = env::var("UNSPLASH_API_KEY").map_err(|_| Error::InvalidApiKey)?;

    let auth = format!("Client-ID {}", api_key);
    let mut auth = HeaderValue::from_str(&auth).map_err(|_| Error::InvalidApiKey)?;
    auth.set_sensitive(true);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth);

    let client = Client::builder().default_headers(headers).build().unwrap();

    let topic = find_topic(&client, "nature").await?;
    println!("{:?}", topic);

    let photos = fetch_photos(&client, &topic).await?;
    println!("{:?}", photos);

    let images = download_photos(&client, &photos).await;
    for (index, image) in images.iter().enumerate() {
        if let Ok(bytes) = image {
            let mut file = File::create(format!("photo{}.png", index)).unwrap();
            file.write_all(bytes.as_ref()).unwrap()
        }
    }

    Ok(())
}
