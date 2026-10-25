use crate::endpoints::Endpoint;
use crate::models::Photo;
use crate::models::photo::{ContentFilter, Download, Orientation};
use http::Method;
use serde::Serialize;
use std::borrow::Cow;
use std::num::NonZeroU32;

#[derive(Debug, Serialize)]
pub struct List {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<NonZeroU32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<NonZeroU32>,
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
            page: Some(NonZeroU32::new(1).unwrap()),
            per_page: Some(NonZeroU32::new(10).unwrap()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Get<'a> {
    #[serde(skip_serializing)]
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
pub struct Random<'a> {
    #[serde(rename = "collections", skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<&'a str>,

    #[serde(rename = "topics", skip_serializing_if = "Option::is_none")]
    pub topic_ids: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_filter: Option<ContentFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<NonZeroU32>,
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
            collection_ids: None,
            topic_ids: None,
            username: None,
            query: None,
            orientation: None,
            content_filter: Some(ContentFilter::Low),
            count: Some(NonZeroU32::new(1).unwrap()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TrackDownload<'a> {
    #[serde(skip_serializing)]
    pub id: &'a str,
}

impl Endpoint for TrackDownload<'_> {
    type Response = Download;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("photos/{}/download", self.id).into()
    }
}
