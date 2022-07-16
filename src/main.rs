extern crate dotenv;

mod anime;
mod fs_scan;
mod metadata;
mod config;
mod database;

use dotenv::dotenv;
use crate::anime::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut animes = fs_scan::scan();

    for anime in animes.collection.iter_mut() {
        println!("Searching metadata for: {}", anime.folder_title);
        match metadata::search_anime_metadata(&anime.folder_title).await {
            Ok(id) => update_anime_metadata(id, anime).await?,
            Err(e) => println!("{:?}", e)
        }        
    }

    database::save_to_data_folder(animes).await?;
    Ok(())
}

async fn update_anime_metadata(id: u32, anime: &mut Anime) -> Result<(), Box<dyn std::error::Error>> {
    let anime_metadata = metadata::get_anime_metadata(id).await?;
    anime.id = Some(id);
    println!("Description: {}", anime_metadata.overview);
    anime.description = anime_metadata.overview;
    println!("Title: {}", anime_metadata.name);
    anime.title = anime_metadata.name;
    if let Some(backdrop) = &anime_metadata.backdrop_path {
        anime.backdrop = strip_base_path(&metadata::download_metadata_image(
            backdrop,
            anime.path.join("backdrop")
        ).await?);
    }
    if let Some(poster) = &anime_metadata.poster_path {
        anime.poster = strip_base_path(&metadata::download_metadata_image(
            poster,
            anime.path.join("poster")
        ).await?);
    }

    for season in anime.seasons.iter_mut() {
        println!("Updating metadata for season {}", season.number);
        let season_metadata = metadata::get_anime_season_metadata(id, season.number).await?;
        if let Some(poster) = &season_metadata.poster_path {
            season.poster = strip_base_path(&metadata::download_metadata_image(
                poster,
                season.path.join("poster")
            ).await?);
        }

        for episode in season.episodes.iter_mut() {
            println!("Updating metadata for episode {}", episode.number);
            let episode_metadata = season_metadata.episodes.iter()
                .find(|&ep| ep.episode_number == episode.number);
            if let Some(metadata) = episode_metadata {
                println!("Title: {}", metadata.name);
                episode.title = String::from(&metadata.name);
                println!("Description: {}", metadata.overview);
                episode.description = String::from(&metadata.overview);
                if let Some(still_path) = &metadata.still_path {
                    episode.thumbnail = strip_base_path(&metadata::download_metadata_image(
                        still_path,
                        episode.path.parent().unwrap().join("metadata").join(format!("{:02}", episode.number))
                    ).await?);
                }
            }
        }
    }
    
    Ok(())
}

