use serde::{Deserialize, Serialize};

pub const UNSPLASH_CLIENT_ID: &str = "s6ac0WqSKlmOLHtY-hlMKzMhywE4vw3aXT9Q9gHzyg8";
pub const UNSPLASH_AUTH_URI: &str = "https://unsplash.com/oauth/authorize";
pub const BACKDROP_AUTH_URI: &str ="https://backdrop-auth-proxy.ohhboi427.workers.dev/exchange";

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequest<'a> {
    pub client_id: &'a str,
    pub redirect_uri: &'a str,
    pub response_type: &'a str,
    pub scope: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenRequest<'a> {
    pub redirect_uri: &'a str,
    pub code: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub created_at: u64,
}
