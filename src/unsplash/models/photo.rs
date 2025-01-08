use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    id: String,
    urls: HashMap<String, String>,
    links: HashMap<String, String>,
}

impl Photo {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn file_url(&self) -> &str {
        &self.urls["raw"]
    }

    pub fn download_track_url(&self) -> &str {
        &self.links["download_location"]
    }
}
