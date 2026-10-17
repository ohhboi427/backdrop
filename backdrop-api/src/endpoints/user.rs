use crate::endpoints::Endpoint;
use crate::models::User;
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
