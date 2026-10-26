use crate::endpoints::Endpoint;
use crate::models::photo::Orientation;
use crate::models::{Photo, User};
use http::Method;
use serde::Serialize;
use std::borrow::Cow;
use std::num::NonZeroU32;

#[derive(Debug, Serialize)]
pub struct Get<'a> {
    #[serde(skip_serializing)]
    pub username: &'a str,
}

impl Endpoint for Get<'_> {
    type Response = User;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}", self.username).into()
    }
}

impl<'a> Get<'a> {
    pub fn new(username: &'a str) -> Self {
        Self { username }
    }
}

#[derive(Debug, Serialize)]
pub struct PhotosOf<'a> {
    #[serde(skip_serializing)]
    pub username: &'a str,

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
        format!("users/{}/photos", self.username).into()
    }
}

impl<'a> PhotosOf<'a> {
    pub fn new(username: &'a str) -> Self {
        Self {
            username,
            page: None,
            per_page: None,
            orientation: None,
        }
    }
}
