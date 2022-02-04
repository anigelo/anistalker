use std::path::PathBuf;
use serde::{Deserialize,Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeSeason {
    pub(crate) season: u8,
    pub(crate) episodes: Vec<AnimeEpisode>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeEpisode {
    pub(crate) number: u8,
    pub(crate) title: String,
    pub(crate) path: PathBuf
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    pub(crate) title: String,
    pub(crate) seasons: Vec<AnimeSeason>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeCollection {
    pub(crate) collection: Vec<Anime>
}