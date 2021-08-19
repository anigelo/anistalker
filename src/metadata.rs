mod kitsu;

use std::path::PathBuf;
use std::error::Error;
use std::io::{ErrorKind, Write};
use std::fs::File;
use kitsu::KitsuMediaCollection;

const METADATA_FOLDER: &str = "metadata";
const METADATA_FILE: &str = "info.json";
const SEARCH_KITSU_URL: &str = "https://kitsu.io/api/edge/anime?filter[text]=";

pub async fn add_metadata(title_path: &PathBuf) -> Result<(), Box<dyn Error>> {

    let folder_name = title_path
        .file_name()
        .ok_or(std::io::Error::from(ErrorKind::NotFound))?
        .to_str()
        .ok_or(std::io::Error::from(ErrorKind::InvalidData))?;

    let metadata_folder_path = title_path.join(METADATA_FOLDER);
    std::fs::create_dir_all(&metadata_folder_path)?;

    add_title_metadata(folder_name, &metadata_folder_path).await?;

    Ok(())
}

async fn add_title_metadata(title: &str, metadata_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let metadata_file_path = metadata_path.join(METADATA_FILE);
    if metadata_file_path.exists() {
        println!("{:?} already exists. Skipping...", metadata_file_path);
        return Ok(());
    }
    println!("Creating {:?}", metadata_file_path);

    let url = format!("{}{}", SEARCH_KITSU_URL, urlencoding::encode(title));
    println!("Fetching metadata from {}", url);
    let res = reqwest::get(url)
        .await?
        .json::<KitsuMediaCollection>()
        .await?;

    let closest_metadata = res.data.first().ok_or(std::io::Error::from(ErrorKind::NotFound))?;
    println!("Got: {:?}", closest_metadata);

    let mut metadata_file = File::create(metadata_file_path)?;
    let closest_metadata = serde_json::to_string_pretty(closest_metadata)?;
    metadata_file.write_all(closest_metadata.as_bytes())?;

    Ok(())
}