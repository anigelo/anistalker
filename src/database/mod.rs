use std::collections::HashMap;
use std::error::Error;
use mongodb::bson::doc;
use mongodb::Client;
use mongodb::options::ClientOptions;
use futures::stream::TryStreamExt;
use crate::prelude::*;

const DB: &str = "anigelo";
const COLLECTION: &str = "anime";
const APP_NAME: &str = "AniStalker";

pub async fn save_to_data_folder(animes: AnimeCollection) -> Result<(), Box<dyn Error>> {
    let mut options = ClientOptions::parse(get_connection_string()).await?;
    options.app_name = Some(APP_NAME.to_string());

    let collection = Client::with_options(options)?
        .database(DB)
        .collection::<Anime>(COLLECTION);

    let mut local_collection: HashMap<_, _> = animes.collection
        .into_iter()
        .map(|anime| (anime.title.to_string(), anime))
        .collect();

    let mut cursor = collection.find(doc! {}, None).await?;
    while let Some(anime_db) = cursor.try_next().await? {
        if let Some(anime) = local_collection.get(&anime_db.title) {
            collection.replace_one(doc! { "title": &anime_db.title }, anime, None).await?;
            local_collection.remove(&anime_db.title);
        } else {
            collection.delete_one(doc! { "title": &anime_db.title }, None).await?;
        }
    }

    if !local_collection.is_empty() {
        collection.insert_many(local_collection.values(), None).await?;
    }

    Ok(())
}