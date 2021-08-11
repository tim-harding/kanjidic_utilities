#[macro_use]
extern crate rocket;

use std::collections::HashSet;

use futures::stream::TryStreamExt;
use kanjidic_types::Character;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOptions},
    Client, Collection,
};
use rocket::{
    fairing::{self, AdHoc},
    serde::json::Json,
    Build, Rocket, State,
};

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
    let collection = client.database("kanjidic").collection::<Character>("kanji");
    Ok(rocket.manage(collection))
}

#[get("/kanji/<literal>?<field>&<language>")]
async fn kanji(
    literal: &str,
    field: Vec<Field>,
    language: Vec<String>,
    db: &State<Collection<Character>>,
) -> Result<Json<CharacterResponse>, &'static str> {
    let field: HashSet<_> = field.into_iter().collect();
    let language: HashSet<_> = language.into_iter().collect();
    let filter = doc! {"literal": literal};
    let character = match db.find_one(filter, None).await {
        Ok(Some(character)) => character,
        Ok(None) => return Err("No kanji found for literal"),
        Err(err) => {
            error!("Error reading a kanji: {}", err);
            return Err("Internal error");
        }
    };
    let response = CharacterResponse::new(character, &field, &language);
    Ok(Json(response))
}

#[get("/kanjis?<radical>&<field>&<language>")]
async fn kanjis(
    radical: Vec<String>,
    field: Vec<Field>,
    language: Vec<String>,
    db: &State<Collection<Character>>,
) -> Result<Json<Vec<CharacterResponse>>, &'static str> {
    let field: HashSet<_> = field.into_iter().collect();
    let language: HashSet<_> = language.into_iter().collect();
    let filter = doc! {"decomposition": {"$all": radical}};
    let find_options = {
        let mut find_options = FindOptions::default();
        // Todo: allow a configurable limit
        find_options.limit = Some(10);
        find_options
    };
    let mut cursor = match db.find(filter, find_options).await {
        Ok(cursor) => cursor,
        Err(_) => return Err("No kanji found for radicals"),
    };
    let mut characters = vec![];
    loop {
        match cursor.try_next().await {
            Ok(Some(character)) => {
                let response_part = CharacterResponse::new(character, &field, &language);
                characters.push(response_part);
            }
            Ok(None) => break,
            Err(err) => {
                error!("Error reading a kanji: {}", err);
                return Err("Internal error");
            }
        }
    }
    Ok(Json(characters))
}
