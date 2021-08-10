#[macro_use]
extern crate rocket;

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
use futures::stream::TryStreamExt;

mod character_response;
use character_response::CharacterResponse;
mod field;
use field::Field;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Connect Database", init_db))
        .mount("/", routes![kanji])
}

#[get("/kanji/<literal>?<field>&<language>")]
async fn kanji(
    literal: &str,
    field: Vec<Field>,
    language: Vec<String>,
    db: &State<Collection<Character>>,
) -> Result<Json<CharacterResponse>, &'static str> {
    let filter = doc! {"literal": literal};
    let find_options = {
        let mut find_options = FindOptions::default();
        find_options.limit = Some(1);
        find_options
    };
    let mut cursor = match db.find(filter, find_options).await {
        Ok(cursor) => cursor,
        Err(_) => return Err("No kanji found for literal"),
    };
    let character = match cursor.try_next().await {
        Ok(Some(character)) => character,
        Err(_) | Ok(None) => return Err("No kanji found for literal"),
    };
    let response = CharacterResponse::new(&character, &field, &language);
    Ok(Json(response))
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
