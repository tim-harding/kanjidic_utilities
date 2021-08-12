#[macro_use]
extern crate rocket;

use futures::stream::TryStreamExt;
use kanjidic_types::Character;
use mongodb::{
    bson::{doc, to_bson, Document},
    options::{ClientOptions, FindOneOptions, FindOptions},
    Client, Database,
};
use rocket::{
    fairing::{self, AdHoc},
    serde::json::Json,
    Build, Rocket, State,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

mod character_response;
use character_response::CharacterResponse;
mod field;
use field::Field;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Connect Database", init_db))
        .attach(AdHoc::try_on_ignite("Create cache", init_cache))
        .mount("/", routes![kanji, kanjis])
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct KanjiData {
    character: Character,
    decomposition: Option<HashSet<String>>,
}

type KanjiCache = HashMap<String, KanjiData>;
type RadkCache = HashMap<String, HashSet<String>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Radk {
    radical: String,
    stroke: u8,
    kanji: HashSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Cache {
    kanji: KanjiCache,
    radk: RadkCache,
}

// Reference: https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/databases/src/sqlx.rs
async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
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

async fn init_cache(rocket: Rocket<Build>) -> fairing::Result {
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

#[get("/kanji/<literal>?<field>&<language>")]
async fn kanji(
    literal: &str,
    field: Vec<Field>,
    language: Vec<String>,
    db: &State<Database>,
) -> Result<Json<CharacterResponse>, &'static str> {
    let filter = doc! {"literal": literal};
    let find_options = {
        let mut find_options = FindOneOptions::default();
        find_options.projection = Some(projection(field, language));
        find_options
    };
    let character = match db
        .collection::<CharacterResponse>("kanji")
        .find_one(filter, find_options)
        .await
    {
        Ok(Some(character)) => character,
        Ok(None) => return Err("No kanji found for literal"),
        Err(err) => {
            error!("Error reading a kanji: {}", err);
            return Err("Internal error");
        }
    };
    Ok(Json(character))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct KanjisResponse {
    pub intersection: HashSet<String>,
    pub characters: Vec<CharacterResponse>,
}

#[get("/kanjis?<radical>&<field>&<language>")]
async fn kanjis(
    radical: Vec<String>,
    field: Vec<Field>,
    language: Vec<String>,
    db: &State<Database>,
) -> Result<Json<KanjisResponse>, &'static str> {
    let characters = kanjis_characters(&radical, field, language, db).await?;
    let intersection = kanjis_intersection(&radical, db).await?;
    let response = KanjisResponse {
        intersection,
        characters,
    };
    Ok(Json(response))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
struct Adjacency {
    pub radical: String,
    pub adjacents: Vec<String>,
}

async fn kanjis_intersection(
    radical: &[String],
    db: &Database,
) -> Result<HashSet<String>, &'static str> {
    let filter = doc! {"radical": {"$in": radical}};
    let mut cursor = match db
        .collection::<Adjacency>("adjacency")
        .find(filter, None)
        .await
    {
        Ok(cursor) => cursor,
        Err(err) => {
            error!("adjacency db.find: {}", err);
            return Err("Internal error");
        }
    };
    let mut sets = vec![];
    loop {
        match cursor.try_next().await {
            Ok(Some(adjacency)) => {
                let set: HashSet<_> = adjacency.adjacents.into_iter().collect();
                sets.push(set)
            }
            Ok(None) => break,
            Err(err) => {
                error!("Error reading an intersection: {}", err);
                return Err("Internal error");
            }
        }
    }
    let adjacents: HashSet<_> = match sets.pop() {
        Some(seed) => seed
            .into_iter()
            .filter(|k| sets.iter().all(|s| s.contains(k)))
            .collect(),
        None => HashSet::default(),
    };
    Ok(adjacents)
}

async fn kanjis_characters(
    radical: &[String],
    field: Vec<Field>,
    language: Vec<String>,
    db: &Database,
) -> Result<Vec<CharacterResponse>, &'static str> {
    let filter = doc! {"decomposition": {"$all": radical}};
    let find_options = {
        let mut find_options = FindOptions::default();
        // Todo: make this configurable
        find_options.limit = Some(10);
        find_options.projection = Some(projection(field, language));
        find_options
    };
    let mut cursor = match db
        .collection::<CharacterResponse>("kanji")
        .find(filter, find_options)
        .await
    {
        Ok(cursor) => cursor,
        Err(err) => {
            error!("kanjis db.find: {}", err);
            return Err("No kanji found for radicals");
        }
    };
    let mut characters = vec![];
    loop {
        match cursor.try_next().await {
            Ok(Some(character)) => {
                characters.push(character);
            }
            Ok(None) => break,
            Err(err) => {
                error!("Error reading a kanji: {}", err);
                return Err("Internal error");
            }
        }
    }
    Ok(characters)
}

fn projection(field: Vec<Field>, language: Vec<String>) -> Document {
    let mut projection: Document = field
        .into_iter()
        .map(|field| {
            let key = match field {
                Field::Radicals => "radicals",
                Field::Decomposition => "decomposition",
                Field::Translations => "translations",
                Field::Codepoints => "codepoints",
                Field::Grade => "grade",
                Field::StrokeCounts => "stroke_counts",
                Field::Variants => "variants",
                Field::Frequency => "frequency",
                Field::RadicalNames => "radical_names",
                Field::Jlpt => "jlpt",
                Field::References => "references",
                Field::QueryCodes => "query_codes",
                Field::Readings => "readings",
                Field::Nanori => "nanori",
            };
            (key.to_owned(), to_bson(&1).unwrap())
        })
        .collect();
    projection.extend([("literal".to_owned(), to_bson(&1).unwrap())]);
    if language.len() > 0 {
        let translations: Document = language
            .into_iter()
            .map(|lang| (lang, to_bson(&1).unwrap()))
            .collect();
        let _old = projection.insert("translations", translations);
    }
    projection
}
