mod anime;
mod fs_scan;
mod fs_store;
mod metadata;
mod config;

mod prelude {
    pub use crate::config::*;
    pub use crate::anime::*;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut animes = fs_scan::scan();

    for anime in animes.collection.iter_mut() {
        println!("Searching metadata for: {}", anime.folder_title);
        let id = metadata::search_anime_metadata(&anime.folder_title).await?.unwrap();
        
        let anime_metadata = metadata::get_anime_metadata(id).await?;
        println!("Description: {}", anime_metadata.overview);
        anime.description = anime_metadata.overview;
        println!("Title: {}", anime_metadata.name);
        anime.title = anime_metadata.name;
        anime.backdrop = metadata::download_metadata_image(
            &anime_metadata.backdrop_path,
            anime.path.join("backdrop")
        ).await?;
        anime.poster = metadata::download_metadata_image(
            &anime_metadata.poster_path,
            anime.path.join("poster")
        ).await?;

        for season in anime.seasons.iter_mut() {
            println!("Updating metadata for season {}", season.number);
            let season_metadata = metadata::get_anime_season_metadata(id, season.number).await?;
            println!("Description: {}", season_metadata.overview);
            season.description = season_metadata.overview;
            season.poster = metadata::download_metadata_image(
                &season_metadata.poster_path,
                season.path.join("poster")
            ).await?;

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
                        episode.thumbnail = metadata::download_metadata_image(
                            still_path,
                            episode.path.parent().unwrap().join("metadata").join(format!("{:02}", episode.number))
                        ).await?;
                    }
                }
            }
        }
    }

    fs_store::save_to_data_folder(animes)?;
    
    Ok(())
}

