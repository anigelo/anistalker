
use std::fs::DirEntry;
use regex::Regex;
use crate::prelude::*;

pub fn scan(media_path: &str) -> AnimeCollection {
    AnimeCollection {
        collection: get_anime_folders(media_path).unwrap().into_iter()
            .map(|anime| Anime {
                title: anime.path().file_stem().unwrap().to_str().unwrap().to_string(),
                seasons: get_seasons(anime).unwrap_or(vec![])
            }).collect()
    }
}

fn get_episodes(season: DirEntry) -> std::io::Result<Vec<AnimeEpisode>> {
    let episodes: Vec<AnimeEpisode> = season.path().read_dir()?
        .filter_map(|dir| dir.ok())
        .filter(|dir| !dir.path().is_dir())
        .filter_map(|dir| {
            let episode_number = dir.path().file_stem().unwrap().to_str().unwrap().parse();
            if let Ok(episode) = episode_number {
                Some(AnimeEpisode {number: episode, title: String::new(), path: dir.path()})
            } else { None }
        })
        .collect();

    Ok(episodes)
}

fn get_seasons(anime: DirEntry) -> std::io::Result<Vec<AnimeSeason>> {
    let re = Regex::new(r"(S|Season )(\d{2})").unwrap();

    let seasons: Vec<AnimeSeason> = anime.path().read_dir()?
        .filter_map(|dir| dir.ok())
        .filter(|dir| dir.path().is_dir())
        .filter_map(|dir| {
            if let Some(captures) = re.captures(dir.file_name().to_str().unwrap()) {
                if let Some(group) = captures.get(2) {
                    Some(AnimeSeason {episodes: get_episodes(dir).unwrap_or(vec![]), season: group.as_str().parse().unwrap()})
                } else { None }
            } else {
                None
            }
        })
        .collect();

    if seasons.is_empty() {
        Ok(vec![AnimeSeason{season: 1, episodes: get_episodes(anime).unwrap_or(vec![])}])
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