use crate::routes::exchange::ExchangeResponse;
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::RwLock;
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct AppState {
    pub sessions: RwLock<HashMap<Uuid, (Option<ExchangeResponse>, Instant)>>,
    pub server_url: Url,
    pub access_key: String,
    pub secret_key: String,
}
