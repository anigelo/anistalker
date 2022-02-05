use std::fs::DirEntry;
use regex::Regex;
use lazy_static::lazy_static;
use crate::prelude::*;

pub fn scan() -> AnimeCollection {
    AnimeCollection {
        collection: get_anime_folders(&get_media_path()).unwrap().into_iter()
            .map(|anime| Anime::new(
                anime.path(),
                anime.path().file_stem().unwrap().to_str().unwrap().to_string(),
                get_seasons(anime).unwrap_or(vec![])
            )).collect()
    }
}

fn get_episodes(season: DirEntry) -> std::io::Result<Vec<AnimeEpisode>> {
    let episodes: Vec<AnimeEpisode> = season.path().read_dir()?
        .filter_map(|dir| dir.ok())
        .filter(|dir| !dir.path().is_dir())
        .filter_map(|dir| {
            let episode_number = dir.path().file_stem().unwrap().to_str().unwrap().parse();
            if let Ok(episode) = episode_number {
                Some(AnimeEpisode::new(episode, dir.path()))
            } else { None }
        })
        .collect();

    Ok(episodes)
}

fn get_seasons(anime: DirEntry) -> std::io::Result<Vec<AnimeSeason>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(S|Season )(\d{2})").unwrap();
    }

    let seasons: Vec<AnimeSeason> = anime.path().read_dir()?
        .filter_map(|dir| dir.ok())
        .filter(|dir| dir.path().is_dir())
        .filter_map(|dir| {
            if let Some(captures) = RE.captures(dir.file_name().to_str().unwrap()) {
                if let Some(group) = captures.get(2) {
                    Some(AnimeSeason::new(
                        dir.path(),
                        group.as_str().parse().unwrap(), 
                        get_episodes(dir).unwrap_or(vec![])))
                } else { None }
            } else {
                None
            }
        })
        .collect();

    if seasons.is_empty() {
        Ok(vec![AnimeSeason::new(anime.path(), 1, get_episodes(anime).unwrap_or(vec![]))])
    } else {
        Ok(seasons)
    }
}

fn get_anime_folders(media_path: &str) -> std::io::Result<Vec<DirEntry>> {
    let media: Vec<DirEntry> = std::fs::read_dir(media_path)?
        .filter_map(|dir| dir.ok())
        .filter(|dir| dir.path().is_dir())
        .collect();

    Ok(media)
}