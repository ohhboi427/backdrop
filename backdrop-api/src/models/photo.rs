use crate::models::User;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Urls {
    pub raw: Url,
    pub full: Url,
    pub regular: Url,
    pub small: Url,
    pub thumb: Url,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub itself: Url,
    pub html: Url,
    pub download: Url,
    pub download_location: Url,
}

#[derive(Debug, Deserialize)]
pub struct Exif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub name: Option<String>,
    pub exposure_time: Option<String>,
    pub aperture: Option<String>,
    pub focal_length: Option<String>,
    pub iso: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub position: Option<Position>,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Photo {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub color: Option<String>,
    pub blur_hash: Option<String>,
    pub description: Option<String>,
    pub urls: Urls,
    pub slug: Option<String>,
    pub alt_description: Option<String>,
    pub short_description: Option<String>,
    pub seo_meta_description: Option<String>,
    pub links: Links,
    pub user: User,
    pub exif: Option<Exif>,
    pub location: Option<Location>,
}
