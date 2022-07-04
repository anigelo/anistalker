use std::fs::File;
use std::io;
use std::path::PathBuf;
use search_result::*;
use tv_metadata::*;
use crate::prelude::*;

mod search_result;
mod tv_metadata;

const BASE_URL: &str = "https://api.themoviedb.org/3";
const BASE_IMAGE_URL: &str = "https://image.tmdb.org/t/p/original";

fn base_url(endpoint: &str) -> String {
    format!("{base}{endpoint}?api_key={api_key}&language={language}", 
            base = BASE_URL,
            endpoint = endpoint,
            api_key = &get_metadata_api_key(),
            language = &get_lang()
    )
}

pub async fn search_anime_metadata(title: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let request_url = format!("{}&query={}", base_url("/search/tv"), title);
    
    let search: SearchResults = reqwest::get(&request_url).await?.json().await?;

    if let Some(result) = search.results.first() {
        Ok(result.id)
    } else { panic!("No matches") }
}

pub async fn get_anime_metadata(id: u32) -> Result<TvMetadata, reqwest::Error> {
    let request_url = base_url(&format!("/tv/{}", id));

    let metadata: TvMetadata = reqwest::get(&request_url).await?.json().await?;

    Ok(metadata)
}

pub async fn get_anime_season_metadata(id: u32, season: u8) -> Result<TvSeasonMetadata, reqwest::Error> {
    let request_url = base_url(&format!("/tv/{}/season/{}", id, season));

    let metadata: TvSeasonMetadata = reqwest::get(&request_url).await?.json().await?;

    Ok(metadata)
}

pub async fn download_metadata_image(image_endpoint: &str, target_path: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let image_url = format!("{}{}", BASE_IMAGE_URL, image_endpoint);
    
    let extension = image_endpoint.split('.').last().unwrap();
    let target_path = target_path.with_extension(extension);

    println!("Downloading image in: {:?}", target_path);
    let parent = target_path.parent().unwrap();
    std::fs::create_dir_all(parent)?;
    let target = File::options()
        .write(true)
        .create_new(true)
        .open(&target_path);

    match target {
        Ok(mut target) => {
            println!("{:?}", target_path);
            let response = reqwest::get(&image_url).await?.bytes().await?;
            io::copy(&mut response.as_ref(), &mut target)?;
        },
        Err(e) => println!("Writing file '{:?}': {}", target_path, e)
    }

    Ok(target_path)
}