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

impl std::fmt::Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuthToken::Bearer(x) => write!(f, "Bearer {}", x),
            AuthToken::ClientId(x) => write!(f, "Client-ID {}", x),
        }
    }
}
