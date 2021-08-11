#[macro_use]
extern crate rocket;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson, Document},
    options::{ClientOptions, FindOptions},
    Client, Collection,
};
use rocket::{
    fairing::{self, AdHoc},
    log::private::info,
    serde::json::Json,
    Build, Rocket, State,
};
use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, collections::{HashMap, HashSet}};

mod character_response;
use character_response::CharacterResponse;
mod field;
use field::Field;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Connect Database", init_db))
        .mount("/", routes![kanji, kanjis])
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
    let collection = client
        .database("kanjidic")
        .collection::<CharacterResponse>("kanji");
    Ok(rocket.manage(collection))
}

#[get("/kanji/<literal>?<field>&<language>")]
async fn kanji(
    literal: &str,
    field: Vec<Field>,
    language: Vec<String>,
    db: &State<Collection<CharacterResponse>>,
) -> Result<Json<CharacterResponse>, &'static str> {
    let _field: HashSet<_> = field.into_iter().collect();
    let _language: HashSet<_> = language.into_iter().collect();
    let filter = doc! {"literal": literal};
    let character = match db.find_one(filter, None).await {
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
    pub valid_radicals: HashSet<String>,
    pub characters: Vec<CharacterResponse>,
}

#[get("/kanjis?<radical>&<field>&<language>")]
async fn kanjis(
    radical: Vec<String>,
    field: Vec<Field>,
    language: Vec<String>,
    db: &State<Collection<CharacterResponse>>,
) -> Result<Json<KanjisResponse>, &'static str> {
    let filter = doc! {"decomposition": {"$all": radical}};
    let find_options = {
        let mut find_options = FindOptions::default();
        let mut projection: Document = field
            .into_iter()
            .filter_map(|field| {
                let key = match field {
                    Field::Radicals => None,
                    Field::Decomposition => None,
                    Field::Translations => Some("translations"),
                    Field::Codepoints => Some("codepoints"),
                    Field::Grade => Some("grade"),
                    Field::StrokeCounts => Some("stroke_counts"),
                    Field::Variants => Some("variants"),
                    Field::Frequency => Some("frequency"),
                    Field::RadicalNames => Some("radical_names"),
                    Field::Jlpt => Some("jlpt"),
                    Field::References => Some("references"),
                    Field::QueryCodes => Some("query_codes"),
                    Field::Readings => Some("readings"),
                    Field::Nanori => Some("nanori"),
                };
                key.map(|key| (key.to_owned(), to_bson(&1).unwrap()))
            })
            .collect();
        projection.extend([
            ("literal".to_owned(), to_bson(&1).unwrap()),
            ("decomposition".to_owned(), to_bson(&1).unwrap()),
        ]);
        if language.len() > 0 {
            let translations: Document = language
                .into_iter()
                .map(|lang| (lang, to_bson(&1).unwrap()))
                .collect();
            let _old = projection.insert("translations", translations);
        }
        find_options.projection = Some(projection);
        find_options
    };
    let now = std::time::Instant::now();
    let mut cursor = match db.find(filter, find_options).await {
        Ok(cursor) => cursor,
        Err(err) => {
            error!("kanjis db.find: {}", err);
            return Err("No kanji found for radicals");
        }
    };
    info!("db.find elapsed: {}", now.elapsed().as_millis());
    let mut characters = vec![];
    let mut valid_radicals = HashSet::default();
    let now = std::time::Instant::now();
    let mut acc = 0;
    loop {
        match cursor.try_next().await {
            Ok(Some(character)) => {
                let now = std::time::Instant::now();
                if let Some(decomposition) = &character.decomposition {
                    valid_radicals.extend(decomposition.clone().into_iter())
                }
                characters.push(character);
                acc += now.elapsed().as_micros();
            }
            Ok(None) => break,
            Err(err) => {
                error!("Error reading a kanji: {}", err);
                return Err("Internal error");
            }
        }
    }
    info!("cursor.try_next elapsed: {}", now.elapsed().as_millis());
    info!("Process character elapsed: {}", acc / 1000);
    let response = KanjisResponse {
        valid_radicals,
        characters,
    };
    Ok(Json(response))
}
