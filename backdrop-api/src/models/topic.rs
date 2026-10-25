use crate::models::{Photo, User};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Links {
    #[serde(rename = "self")]
    pub itself: Url,
    pub html: Url,
    pub photos: Url,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
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
