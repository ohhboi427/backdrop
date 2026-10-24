use crate::endpoints::Endpoint;
use crate::models::photo::{Color, ContentFilter, Orientation};
use crate::models::{Photo, SearchResult, User};
use http::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Serialize)]
pub struct Users<'a> {
    pub query: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

impl Endpoint for Users<'_> {
    type Response = SearchResult<User>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "search/users".into()
    }
}

#[derive(Debug, Serialize)]
pub struct Photos<'a> {
    pub query: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_filter: Option<ContentFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

impl Endpoint for Photos<'_> {
    type Response = SearchResult<Photo>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "search/photos".into()
    }
}
