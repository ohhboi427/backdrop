use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Social {
    pub instagram_username: Option<String>,
    pub portfolio_url: Option<String>,
    pub twitter_username: Option<String>,
    pub paypal_email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub itself: Url,
    pub photos: Url,
    pub likes: Url,
    pub html: Url,
}

#[derive(Debug, Deserialize)]
pub struct ProfileImage {
    pub small: Url,
    pub medium: Url,
    pub large: Url,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub location: Option<String>,
    pub profile_image: ProfileImage,
    pub links: Links,
    pub social: Social,
}
