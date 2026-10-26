use crate::endpoints::Endpoint;
use crate::models::photo::Orientation;
use crate::models::{Photo, Topic};
use http::Method;
use serde::Serialize;
use std::borrow::Cow;
use std::num::NonZeroU32;

#[derive(Debug, Default, Serialize)]
pub struct List<'a> {
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids_or_slugs: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<NonZeroU32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<NonZeroU32>,
}

impl Endpoint for List<'_> {
    type Response = Vec<Topic>;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "topics".into()
    }
}

impl List<'_> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Serialize)]
pub struct Get<'a> {
    #[serde(skip_serializing)]
    pub id_or_slug: &'a str,
}

impl Endpoint for Get<'_> {
    type Response = Topic;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("topics/{}", self.id_or_slug).into()
    }
}

impl<'a> Get<'a> {
    pub fn new(id_or_slug: &'a str) -> Self {
        Self { id_or_slug }
    }
}

#[derive(Debug, Serialize)]
pub struct PhotosOf<'a> {
    #[serde(skip_serializing)]
    pub id_or_slug: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<NonZeroU32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<NonZeroU32>,

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

impl<'a> PhotosOf<'a> {
    pub fn new(id_or_slug: &'a str) -> Self {
        Self {
            id_or_slug,
            page: None,
            per_page: None,
            orientation: None,
        }
    }
}
