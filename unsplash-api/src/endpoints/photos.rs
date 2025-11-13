use crate::endpoints::Endpoint;
use crate::endpoints::users::User;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Urls {
    raw: String,
    full: String,
    regular: String,
    small: String,
    thumb: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    html: String,
    download: String,
    download_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub color: String,
    pub description: Option<String>,
    pub user: User,
    pub urls: Urls,
    pub links: Links,
}

pub struct List {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl Default for List {
    fn default() -> Self {
        Self {
            page: None,
            per_page: None,
        }
    }
}

impl Endpoint for List {
    type Output = Vec<Photo>;

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "/photos?page={}&per_page={}",
            self.page.unwrap_or(1),
            self.per_page.unwrap_or(10)
        )
        .into()
    }
}

pub struct Get<'a> {
    pub id: &'a str,
}

impl Endpoint for Get<'_> {
    type Output = Photo;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/photos/{}", self.id).into()
    }
}

pub struct TrackDownload<'a> {
    pub id: &'a str,
}

impl Endpoint for TrackDownload<'_> {
    type Output = Photo;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/photos/{}/download", self.id).into()
    }
}
