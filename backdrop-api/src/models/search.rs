use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct SearchResult<T> {
    pub total: u32,
    pub total_pages: u32,
    pub results: Vec<T>,
}
