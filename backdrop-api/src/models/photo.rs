use crate::models::User;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Urls {
    pub raw: Url,
    pub full: Url,
    pub regular: Url,
    pub small: Url,
    pub thumb: Url,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Links {
    #[serde(rename = "self")]
    pub itself: Url,
    pub html: Url,
    pub download: Url,
    pub download_location: Url,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Exif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub name: Option<String>,
    pub exposure_time: Option<String>,
    pub aperture: Option<String>,
    pub focal_length: Option<String>,
    pub iso: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Location {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub position: Option<Position>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Position {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Photo {
    pub id: String,
    pub width: Option<NonZeroU32>,
    pub height: Option<NonZeroU32>,
    pub color: Option<String>,
    pub blur_hash: Option<String>,
    pub description: Option<String>,
    pub urls: Urls,
    pub slug: Option<String>,
    pub alt_description: Option<String>,
    pub short_description: Option<String>,
    pub seo_meta_description: Option<String>,
    pub links: Option<Links>,
    pub user: Option<User>,
    pub exif: Option<Exif>,
    pub location: Option<Location>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct Download {
    pub url: Url,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub enum ContentFilter {
    Low,
    High,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub enum Color {
    BlackAndWhite,
    Black,
    White,
    Yellow,
    Orange,
    Red,
    Purple,
    Magenta,
    Green,
    Teal,
    Blue,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub enum Orientation {
    Landscape,
    Portrait,
    Squarish,
}
