use std::fs;
use std::io::Result;
use std::fs::DirEntry;
use std::path::PathBuf;
use core::result;
use std::error::Error;
use std::ffi::OsString;

pub fn make_episode_folders(title_folder: &DirEntry) -> result::Result<(), Box<dyn Error>> {
    for (i, episode) in read_episodes(&title_folder.path())?.into_iter().enumerate() {
        let format = format!("{:0>2}", i+1);
        if !check_format(&format, &episode.path()) {
            move_episode(&format, &episode.path())?
        }
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

fn check_format(ep_name: &str, ep_path: &PathBuf) -> bool {
    if let Some(name) = ep_path.file_stem() {
        OsString::from(ep_name) == name
    } else {
        false
    }
}

fn move_episode(ep_name: &str, ep_path: &PathBuf) -> Result<()> {
    let mut new_ep_path = PathBuf::from(ep_path)
        .with_file_name(ep_name);

    if let Some(extension) = ep_path.extension() {
        new_ep_path = new_ep_path.with_extension(extension);
    }

    println!("Moved episode to {:?}", new_ep_path);
    fs::rename(ep_path, new_ep_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_format_can_handle_three_digits() {
        for i in 1..=103 {
            let path = PathBuf::from(format!("/{:0>2}.mkv", i));
            println!("{}", i);
            assert!(check_format(&format!("{:0>2}", i), &path));
        }
    }
}
