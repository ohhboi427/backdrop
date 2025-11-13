use crate::endpoints::Endpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Social {
    pub instagram_username: Option<String>,
    pub portfolio_url: Option<String>,
    pub twitter_username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub social: Social,
}

pub struct Me;

impl Endpoint for Me {
    type Output = User;

    fn endpoint(&self) -> Cow<'static, str> {
        "/me".into()
    }
}

pub struct Get<'a> {
    pub username: &'a str,
}

impl Endpoint for Get<'_> {
    type Output = User;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/users/{}", self.username).into()
    }
}
