use std::path::PathBuf;
use std::error::Error;
use serde::{Deserialize,Serialize};
use std::io::{ErrorKind, Write};
use std::fs::File;

const METADATA_FILE: &str = "metadata.anistk";
const SEARCH_KITSU_URL: &str = "https://kitsu.io/api/edge/anime?filter[text]=";

pub async fn add_title_metadata(title_path: &PathBuf) -> Result<(), Box<dyn Error>> {

    let url = format!("{}{}",
        SEARCH_KITSU_URL,
        urlencoding::encode(title_path
            .file_name()
            .ok_or(std::io::Error::from(ErrorKind::NotFound))?
            .to_str().unwrap())
    );

    let metadata_file_path = title_path.join(METADATA_FILE);
    println!("Creating {:?}", metadata_file_path);

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

#[derive(Deserialize, Serialize, Debug)]
struct KitsuMediaCollection {
    data: Vec<KitsuMedia>
}

#[derive(Debug, Serialize, Deserialize)]
struct KitsuMedia {
    id: Option<String>,
    attributes: KitsuMediaAttrs
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct KitsuMediaAttrs {
    canonicalTitle: Option<String>,
    startDate: Option<String>,
    endDate: Option<String>,
    posterImage: Option<KitsuMediaAttrsImages>,
    coverImage: Option<KitsuMediaAttrsImages>,
    synopsis: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct KitsuMediaAttrsImages {
    tiny: Option<String>,
    small: Option<String>,
    medium: Option<String>,
    large: Option<String>,
    original: Option<String>
}