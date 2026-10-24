use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResult<T> {
    pub total: u32,
    pub total_pages: u32,
    pub results: Vec<T>,
}
