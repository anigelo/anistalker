use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResults {
    pub page: u8,
    pub results: Vec<SearchResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u32
}