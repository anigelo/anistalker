use std::env;

const DEFAULT_MEDIA_PATH: &str = "/media";
const DEFAULT_DATA_PATH: &str = "/data";

pub fn get_media_path() -> String {
    env::var("ANIGELO_MEDIA_PATH")
        .unwrap_or(String::from(DEFAULT_MEDIA_PATH))
}

pub fn get_data_path() -> String {
    env::var("ANIGELO_DATA_PATH")
        .unwrap_or(String::from(DEFAULT_DATA_PATH))
}