#[macro_use]
extern crate rocket;

use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use rocket::{Build, Rocket, State, fairing::{self, AdHoc}, log::private::info, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    let collection = client.database("kanjidic").collection::<CharacterResponse>("kanji");
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

// Todo: use FindOptions.projection to limit fields
// and populate CharacterResponse directly

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
    let _field: HashSet<_> = field.into_iter().collect();
    let _language: HashSet<_> = language.into_iter().collect();
    let filter = doc! {"decomposition": {"$all": radical}};
    let now = std::time::Instant::now();
    let mut cursor = match db.find(filter, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Err("No kanji found for radicals"),
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
