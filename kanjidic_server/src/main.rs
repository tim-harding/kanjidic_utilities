#[macro_use]
extern crate rocket;

use std::collections::HashSet;

use futures::stream::TryStreamExt;
use kanjidic_types::{
    Character, Codepoint, Grade, QueryCode, Radical, Reading, Reference, StrokeCount, Translations,
    Variant,
};
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
use serde::{Deserialize, Serialize};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Connect Database", init_db))
        .mount("/", routes![kanji])
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
struct CharacterResponse {
    pub literal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codepoints: Option<Vec<Codepoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radicals: Option<Vec<Radical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<Grade>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_counts: Option<StrokeCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<Variant>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radical_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jlpt: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<Reference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_codes: Option<Vec<QueryCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readings: Option<Vec<Reading>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<Translations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanori: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decomposition: Option<Vec<String>>,
}

impl From<(Character, Fields)> for CharacterResponse {
    fn from(value: (Character, Fields)) -> Self {
        let mut out = CharacterResponse::default();
        out.literal = value.0.literal;
        if value.1.fields.contains(&Field::Codepoints) {
            out.codepoints = Some(value.0.codepoints);
        }
        if value.1.fields.contains(&Field::Radicals) {
            out.radicals = Some(value.0.radicals);
        }
        if value.1.fields.contains(&Field::Grade) {
            out.grade = value.0.grade;
        }
        if value.1.fields.contains(&Field::StrokeCounts) {
            out.stroke_counts = Some(value.0.stroke_counts);
        }
        if value.1.fields.contains(&Field::Variants) {
            out.variants = Some(value.0.variants);
        }
        if value.1.fields.contains(&Field::Frequency) {
            out.frequency = value.0.frequency;
        }
        if value.1.fields.contains(&Field::RadicalNames) {
            out.radical_names = Some(value.0.radical_names);
        }
        if value.1.fields.contains(&Field::Jlpt) {
            out.jlpt = value.0.jlpt;
        }
        if value.1.fields.contains(&Field::References) {
            out.references = Some(value.0.references);
        }
        if value.1.fields.contains(&Field::QueryCodes) {
            out.query_codes = Some(value.0.query_codes);
        }
        if value.1.fields.contains(&Field::Readings) {
            out.readings = Some(value.0.readings);
        }
        if value.1.fields.contains(&Field::Nanori) {
            out.nanori = Some(value.0.nanori);
        }
        if value.1.fields.contains(&Field::Decomposition) {
            out.decomposition = value.0.decomposition;
        }

        if value.1.fields.contains(&Field::Translations) {
            let mut translations = Translations::default();
            for (language, localization) in value.0.translations.iter() {
                if value.1.languages.contains(language) {
                    translations.insert(language.into(), localization.clone());
                }
            }
            out.translations = Some(translations);
        }

        out
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Hash)]
enum Field {
    Codepoints,
    Radicals,
    Grade,
    StrokeCounts,
    Variants,
    Frequency,
    RadicalNames,
    Jlpt,
    References,
    QueryCodes,
    Readings,
    Translations,
    Nanori,
    Decomposition,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
struct Fields {
    pub fields: HashSet<Field>,
    pub languages: Vec<String>,
}

#[get("/kanji/<literal>", data = "<fields>")]
async fn kanji(
    literal: &str,
    fields: Json<Fields>,
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
    let response = CharacterResponse::from((character, fields.to_owned()));
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
