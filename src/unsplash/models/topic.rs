use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    id: String,
}

impl Topic {
    pub fn id(&self) -> &str {
        &self.id
    }
}
