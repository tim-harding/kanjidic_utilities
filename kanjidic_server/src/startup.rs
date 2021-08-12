use std::collections::HashSet;
use futures::stream::TryStreamExt;
use kanjidic_types::Character;
use mongodb::{Client, Database, bson::doc, options::ClientOptions};
use rocket::{Build, Rocket, fairing};
use crate::cache::{Cache, KanjiCache, KanjiData, Radk, RadkCache};

pub async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    let db_url = match std::env::var("mongodb_url") {
        Ok(url) => url,
        Err(err) => {
            error!("Failed to get `mongodb_url` environment variable: {}", err);
            return Err(rocket);
        }
    };
    let client_options = match ClientOptions::parse(db_url).await {
        Ok(options) => options,
        Err(err) => {
            error!("Failed to parse mongodb client options: {}", err);
            return Err(rocket);
        }
    };
    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to get mongodb client: {}", err);
            return Err(rocket);
        }
    };
    let database = client.database("kanjidic");
    Ok(rocket.manage(database))
}

pub async fn init_cache(rocket: Rocket<Build>) -> fairing::Result {
    let db = rocket.state::<Database>().unwrap();
    let kanji = match get_kanji_data(db).await {
        Ok(kanji) => kanji,
        Err(()) => return Err(rocket),
    };
    let radk = match get_radk_data(db).await {
        Ok(radk) => radk,
        Err(()) => return Err(rocket),
    };
    let cache = Cache { kanji, radk };
    Ok(rocket.manage(cache))
}

async fn get_radk_data(db: &Database) -> Result<RadkCache, ()> {
    let filter = doc! {};
    let mut cursor = match db.collection::<Radk>("radk").find(filter, None).await {
        Ok(cursor) => cursor,
        Err(err) => {
            error!("radk db.find: {}", err);
            return Err(());
        }
    };
    let mut cache = RadkCache::default();
    loop {
        match cursor.try_next().await {
            Ok(Some(radk)) => {
                let Radk { kanji, radical, .. } = radk;
                let kanji: HashSet<_> = kanji.into_iter().collect();
                cache.insert(radical, kanji);
            }
            Ok(None) => break,
            Err(err) => {
                error!("Error reading an radk: {}", err);
                return Err(());
            }
        }
    }
    Ok(cache)
}

async fn get_kanji_data(db: &Database) -> Result<KanjiCache, ()> {
    let filter = doc! {};
    let mut cursor = match db.collection::<Character>("kanji").find(filter, None).await {
        Ok(cursor) => cursor,
        Err(err) => {
            error!("adjacency db.find: {}", err);
            return Err(());
        }
    };
    let mut kanji = KanjiCache::default();
    loop {
        match cursor.try_next().await {
            Ok(Some(character)) => {
                let literal = character.literal.clone();
                let decomposition: Option<HashSet<_>> = character
                    .decomposition
                    .clone()
                    .map(|d| d.into_iter().collect());
                let data = KanjiData {
                    character,
                    decomposition,
                };
                kanji.insert(literal, data);
            }
            Ok(None) => break,
            Err(err) => {
                error!("Error reading a kanji: {}", err);
                return Err(());
            }
        }
    }
    Ok(kanji)
}