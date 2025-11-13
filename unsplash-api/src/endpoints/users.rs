use crate::endpoints::Endpoint;
use crate::endpoints::photos::Photo;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub instagram_username: Option<String>,
    pub portfolio_url: Option<String>,
    pub twitter_username: Option<String>,
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

pub struct Photos<'a> {
    pub username: &'a str,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl Endpoint for Photos<'_> {
    type Output = Vec<Photo>;

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "/users/{}/photos?page={}&per_page={}",
            self.username,
            self.page.unwrap_or(1),
            self.per_page.unwrap_or(10)
        )
        .into()
    }
}

pub struct LikedPhotos<'a> {
    pub username: &'a str,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl Endpoint for LikedPhotos<'_> {
    type Output = Vec<Photo>;

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "/users/{}/likes?page={}&per_page={}",
            self.username,
            self.page.unwrap_or(1),
            self.per_page.unwrap_or(10)
        )
        .into()
    }
}
