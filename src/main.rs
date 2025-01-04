use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::collections::HashMap;

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
            $(($key, $value.to_string())),*
        ]
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct Topic {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Photo {
    id: String,
    urls: HashMap<String, String>,
    links: HashMap<String, String>,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Missing or invalid Unsplash access key")]
    InvalidApiKey,

    #[error("HTTP error {0}")]
    StatusError(reqwest::StatusCode),
}

type Result<T> = std::result::Result<T, Error>;

fn handle_response(response: Response) -> Result<Response> {
    if !response.status().is_success() {
        return Err(Error::StatusError(response.status()));
    }

    Ok(response)
}

async fn find_topic<T: AsRef<str>>(client: &Client, id_or_slug: T) -> Result<Topic> {
    let response = client
        .get(unsplash_api!("/topics/{}", id_or_slug.as_ref()))
        .send()
        .await
        .unwrap();

    let response = handle_response(response)?;

    Ok(response.json().await.unwrap())
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
        .unwrap();

    let response = handle_response(response)?;

    Ok(response.json().await.unwrap())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("UNSPLASH_API_KEY").map_err(|_| Error::InvalidApiKey)?;

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

    Ok(())
}
