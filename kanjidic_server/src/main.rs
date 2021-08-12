#[macro_use]
extern crate rocket;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson, Document},
    options::{FindOneOptions, FindOptions},
    Database,
};
use rocket::{
    fairing::{AdHoc},
    serde::json::Json, State,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};

mod character_response;
use character_response::CharacterResponse;
mod field;
use field::Field;
mod cache;
mod startup;
use startup::{init_cache, init_db};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Connect Database", init_db))
        .attach(AdHoc::try_on_ignite("Create cache", init_cache))
        .mount("/", routes![kanji, kanjis])
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
