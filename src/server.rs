use std::fs;
use std::io::{Result};
use std::fs::{DirEntry};
use std::path::PathBuf;
use core::result;
use std::error::Error;

pub fn make_episode_folders(title_folder: &DirEntry) -> result::Result<(), Box<dyn Error>> {
    for (i, episode) in read_episodes(&title_folder.path())?.into_iter().enumerate() {
        move_episode(i+1, &episode.path())?
    }
    Ok(())
}

pub fn read_media_folder(media_path: &str) -> Result<Vec<DirEntry>> {
    fs::read_dir(media_path)?
        .map(|dir| dir)
        .collect()
}

fn read_episodes(path: &PathBuf) -> Result<Vec<DirEntry>> {
    let mut episodes = path
        .read_dir()?
        .filter(|ep|
            if let Ok(ep) = ep {
                if let Some(ext) = ep.path().extension() {
                    ext == "mp4" || ext == "mkv" || ext == "avi"
                } else {false}
            } else {false})
        .collect::<Result<Vec<DirEntry>>>()?;

    episodes.sort_by_key(|k| k.path());

    Ok(episodes)
}

fn move_episode(ep_number: usize, ep_path: &PathBuf) -> Result<()> {
    let episode_folder = PathBuf::from(ep_path.parent().unwrap())
        .join(format!("{:0>3}", ep_number));

    fs::create_dir(&episode_folder)?;

    let new_ep_path = episode_folder
        .join(ep_path.file_name().unwrap());

    println!("Moved episode to {:?}", new_ep_path);
    fs::rename(ep_path, new_ep_path)
}

