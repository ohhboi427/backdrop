use crate::endpoints::Endpoint;
use crate::models::photo::Photo;
use http::Method;
use serde::Serialize;
use std::borrow::Cow;
use url::Url;

#[derive(Debug, Serialize)]
pub struct List {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

impl Endpoint for List {
    type Response = Vec<Photo>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "photos".into()
    }
}

impl Default for List {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Get<'a> {
    pub id: &'a str,
}

impl Endpoint for Get<'_> {
    type Response = Photo;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("photos/{}", self.id).into()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Orientation {
    Landscape,
    Portrait,
    Squarish,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentFilter {
    Low,
    High,
}

#[derive(Debug, Serialize)]
pub struct Random<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_filter: Option<ContentFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

impl Endpoint for Random<'_> {
    type Response = Vec<Photo>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "photos/random".into()
    }
}

impl Default for Random<'_> {
    fn default() -> Self {
        Self {
            collections: None,
            topics: None,
            username: None,
            query: None,
            orientation: None,
            content_filter: Some(ContentFilter::Low),
            count: Some(1),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TrackDownload<'a> {
    pub id: &'a str,
}

impl Endpoint for TrackDownload<'_> {
    type Response = Url;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("photos/{}/download", self.id).into()
    }
}
