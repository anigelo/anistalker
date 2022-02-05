use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TvMetadata {
    pub backdrop_path: String,
    pub poster_path: String,
    pub overview: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TvSeasonMetadata {
    pub id: u32,
    pub poster_path: String,
    pub overview: String,
    pub episodes: Vec<TvEpisodeMetadata>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TvEpisodeMetadata {
    pub episode_number: u8,
    pub id: u32,
    pub overview: String,
    pub still_path: String
}