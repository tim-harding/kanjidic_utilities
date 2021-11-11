use crate::cache::{Cache, KanjiCache, Radk, RadkCache, TranslationCache};
use futures::stream::TryStreamExt;
use kanjidic_types::Character;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use rocket::{fairing, Build, Rocket};

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
    let (kanji, translations) = match get_kanji_data(db).await {
        Ok(kanji) => kanji,
        Err(()) => return Err(rocket),
    };
    let radk = match get_radk_data(db).await {
        Ok(radk) => radk,
        Err(()) => return Err(rocket),
    };
    let cache = Cache {
        kanji,
        translations,
        radk,
    };
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
                cache.insert(radk.radical, radk);
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

async fn get_kanji_data(db: &Database) -> Result<(KanjiCache, TranslationCache), ()> {
    let filter = doc! {};
    let mut cursor = match db.collection::<Character>("kanji").find(filter, None).await {
        Ok(cursor) => cursor,
        Err(err) => {
            error!("adjacency db.find: {}", err);
            return Err(());
        }
    };
    let mut kanji = KanjiCache::default();
    let mut translations = TranslationCache::default();
    loop {
        match cursor.try_next().await {
            Ok(Some(character)) => {
                let literal = character.literal;
                for language in character.translations.values() {
                    for translation in language.iter() {
                        for part in translation.split(' ') {
                            match translations.entry(part.to_owned()) {
                                std::collections::hash_map::Entry::Occupied(mut entry) => {
                                    entry.get_mut().push(literal);
                                }
                                std::collections::hash_map::Entry::Vacant(entry) => {
                                    entry.insert(vec![literal]);
                                }
                            }
                        }
                    }
                }
                kanji.insert(literal, character);
            }
            Ok(None) => break,
            Err(err) => {
                error!("Error reading a kanji: {}", err);
                return Err(());
            }
        }
    }
    Ok((kanji, translations))
}
