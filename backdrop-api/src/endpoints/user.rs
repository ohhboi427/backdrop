use crate::endpoints::Endpoint;
use crate::models::{Photo, User, photo::Orientation};
use http::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Serialize)]
pub struct Get<'a> {
    pub username: &'a str,
}

impl Endpoint for Get<'_> {
    type Response = User;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        format!("users/{}", self.username).into()
    }
}

#[derive(Debug, Serialize)]
pub struct PhotosOf<'a> {
    pub username: &'a str,

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
        format!("users/{}/photos", self.username).into()
    }
}
