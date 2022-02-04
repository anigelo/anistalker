use std::{fs, io};
use std::path::Path;
use crate::prelude::*;

pub fn save_to_data_folder(collection: AnimeCollection) -> io::Result<()> {
    let json = serde_json::to_string(&collection).unwrap();
    let index_file = Path::new(&get_data_path()).join("index.json");
    fs::write(index_file, json)
}