use crate::endpoints::Endpoint;
use crate::models::photo::Orientation;
use crate::models::{Photo, Topic};
use http::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Serialize)]
pub struct List<'a> {
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids_or_slugs: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

impl Endpoint for List<'_> {
    type Response = Vec<Topic>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "topics".into()
    }
}

#[derive(Debug, Serialize)]
pub struct Get<'a> {
    pub id_or_slug: &'a str,
}

impl Endpoint for Get<'_> {
    type Response = Topic;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("/topics/{}", self.id_or_slug).into()
    }
}

#[derive(Debug, Serialize)]
pub struct PhotosOf<'a> {
    pub id_or_slug: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

impl Endpoint for PhotosOf<'_> {
    type Response = Vec<Photo>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("topics/{}/photos", self.id_or_slug).into()
    }
}
