use serde::{Deserialize,Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct KitsuMediaCollection {
    pub data: Vec<KitsuMedia>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuMedia {
    id: Option<String>,
    attributes: KitsuMediaAttrs
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct KitsuMediaAttrs {
    canonicalTitle: Option<String>,
    startDate: Option<String>,
    endDate: Option<String>,
    posterImage: Option<KitsuMediaAttrsImages>,
    coverImage: Option<KitsuMediaAttrsImages>,
    synopsis: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct KitsuMediaAttrsImages {
    tiny: Option<String>,
    small: Option<String>,
    medium: Option<String>,
    large: Option<String>,
    original: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KitsuEpisodeCollection {
    pub data: Vec<KitsuEpisode>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuEpisode {
    attributes: KitsuEpisodeAttrs
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct KitsuEpisodeAttrs {
    number: Option<usize>,
    canonicalTitle: Option<String>,
    airdate: Option<String>,
    thumbnail: Option<KitsuMediaAttrsImages>,
    synopsis: Option<String>
}