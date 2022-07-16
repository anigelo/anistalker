use std::path::PathBuf;
use serde::{Deserialize,Serialize};
use crate::config;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeSeason {
    pub(crate) number: u8,
    pub(crate) path: PathBuf,
    pub(crate) episodes: Vec<AnimeEpisode>,
    pub(crate) poster: PathBuf
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeEpisode {
    pub(crate) number: u8,
    pub(crate) title: String,
    pub(crate) path: PathBuf,
    pub(crate) video: PathBuf,
    pub(crate) description: String,
    pub(crate) thumbnail: PathBuf
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    pub(crate) id: Option<u32>,
    pub(crate) folder_title: String,
    pub(crate) title: String,
    pub(crate) path: PathBuf,
    pub(crate) seasons: Vec<AnimeSeason>,
    pub(crate) backdrop: PathBuf,
    pub(crate) poster: PathBuf,
    pub(crate) description: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeCollection {
    pub(crate) collection: Vec<Anime>
}

impl Anime {
    pub fn new(path: PathBuf, folder_title: String, seasons: Vec<AnimeSeason>) -> Anime {
        Anime {
            folder_title, seasons, path,
            title: String::new(),
            poster: PathBuf::new(),
            backdrop: PathBuf::new(),
            description: String::new(),
            id: None
        }
    }
}

impl AnimeSeason {
    pub fn new(path: PathBuf, number: u8, episodes: Vec<AnimeEpisode>) -> AnimeSeason {
        AnimeSeason {
            number, episodes, path,
            poster: PathBuf::new()
        }
    }
}

impl AnimeEpisode {
    pub fn new(number: u8, path: PathBuf) -> AnimeEpisode {
        AnimeEpisode {
            video: strip_base_path(&path),
            path, number, 
            title: String::new(),
            description: String::new(),
            thumbnail: PathBuf::new()
        }
    }
}

pub fn strip_base_path(path: &PathBuf) -> PathBuf {
    let media_path = config::get_media_path();

    match path.strip_prefix(media_path) {
        Ok(new_path) => new_path.to_path_buf(),
        Err(err) => {
            eprintln!("Error striping path for '{:#?}': {:#?}", path, err);
            path.to_path_buf()
        }
    }
}