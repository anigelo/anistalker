use std::{fs, io};
use std::path::{Path, PathBuf};
use crate::prelude::*;

const INDEX_FILE: &str = "index.json";

pub fn save_to_data_folder(collection: AnimeCollection) -> io::Result<()> {
    let json = serde_json::to_string(&collection).unwrap();
    fs::write(get_index_path(), json)
}

#[allow(dead_code)]
pub fn get_anime_collection() -> io::Result<AnimeCollection> {
    let anime_collection = fs::read_to_string(get_index_path())?;
    let anime_collection = serde_json::from_str(&anime_collection).unwrap();
    
    Ok(anime_collection)
}

fn get_index_path() -> PathBuf {
    Path::new(&get_data_path()).join(INDEX_FILE)
}