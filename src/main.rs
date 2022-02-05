mod anime;
mod fs_scan;
mod fs_store;
mod metadata;
mod config;

mod prelude {
    pub use crate::config::*;
    pub use crate::anime::*;
}

use prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animes = fs_scan::scan();

    fs_store::save_to_data_folder(animes).unwrap();
    
    let animes = fs_store::get_anime_collection().unwrap();

    for anime in animes.collection.into_iter() {
        let id = metadata::search_anime_metadata(&anime.title).await?.unwrap();
        
        let anime_metadata = metadata::get_anime_metadata(id).await?;
        println!("{:?}", anime_metadata);
        
        let season_metadata = metadata::get_anime_season_metadata(id, 1).await?;
        println!("{:?}", season_metadata);
        
        let target =  Path::new(&get_data_path()).join("backdrop");
        metadata::download_metadata_image(&anime_metadata.backdrop_path, target).await?;
    }
    
    Ok(())
}

