use crate::endpoints::Endpoint;
use crate::models::User;
use http::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Default, Serialize)]
pub struct Get {}

impl Endpoint for Get {
    type Response = User;

    const METHOD: Method = Method::GET;

    fn path(&self) -> Cow<'static, str> {
        "me".into()
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Update<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_username: Option<&'a str>,
}

impl Endpoint for Update<'_> {
    type Response = User;

    const METHOD: Method = Method::PUT;

    fn path(&self) -> Cow<'static, str> {
        "me".into()
    }
}
