mod server;
mod metadata;

use std::error::Error;

const MEDIA_PATH: &str = "/nas/nfs/docker/jellyfin/media/anime";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    for dir in server::read_media_folder(MEDIA_PATH)? {
        server::make_episode_folders(&dir)?;
        metadata::add_title_metadata(&dir.path()).await?;
    }
    Ok(())
}
