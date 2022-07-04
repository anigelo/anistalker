use std::env;

const DEFAULT_MEDIA_PATH: &str = "/media";
const DEFAULT_LANG: &str = "en-US";

pub fn get_media_path() -> String {
    env::var("ANIGELO_MEDIA_PATH")
        .unwrap_or(String::from(DEFAULT_MEDIA_PATH))
}

pub fn get_metadata_api_key() -> String {
    env::var("ANIGELO_METADATA_API_KEY").expect("METADATA API KEY IS MISSING")
}

pub fn get_connection_string() -> String {
    env::var("ANIGELO_CONNECTION_STRING").expect("DB CONNECTION STRING IS MISSING")
}

pub fn get_lang() -> String {
    env::var("ANIGELO_LANG")
        .unwrap_or(String::from(DEFAULT_LANG))
}