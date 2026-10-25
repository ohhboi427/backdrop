use crate::models::{Photo, User};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub itself: Url,
    pub html: Url,
    pub photos: Url,
}

#[derive(Debug, Deserialize)]
pub struct Topic {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub links: Links,
    pub owners: Vec<User>,
    pub cover_photo: Option<Photo>,
    pub preview_photos: Vec<Photo>,
}
