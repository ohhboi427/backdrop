use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub session_id: Uuid,
    pub auth_url: Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthToken {
    Bearer(String),
    ClientId(String),
}
