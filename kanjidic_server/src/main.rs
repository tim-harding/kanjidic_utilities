#[macro_use]
extern crate rocket;

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

impl CharacterResponse {
    fn new(character: &Character, fields: &[Field], languages: &[String]) -> Self {
        if fields.len() == 0 {
            Self::all_fields(character, languages)
        } else {
            Self::filtered_fields(character, fields, languages)
        }
    }

    fn filtered_fields(character: &Character, fields: &[Field], languages: &[String]) -> Self {
        let mut out = CharacterResponse::default();
        out.literal = character.literal.clone();
        if fields.contains(&Field::Codepoints) {
            out.codepoints = Some(character.codepoints.clone());
        }
        if fields.contains(&Field::Radicals) {
            out.radicals = Some(character.radicals.clone());
        }
        if fields.contains(&Field::Grade) {
            out.grade = character.grade;
        }
        if fields.contains(&Field::StrokeCounts) {
            out.stroke_counts = Some(character.stroke_counts.clone());
        }
        if fields.contains(&Field::Variants) {
            out.variants = Some(character.variants.clone());
        }
        if fields.contains(&Field::Frequency) {
            out.frequency = character.frequency;
        }
        if fields.contains(&Field::RadicalNames) {
            out.radical_names = Some(character.radical_names.clone());
        }
        if fields.contains(&Field::Jlpt) {
            out.jlpt = character.jlpt;
        }
        if fields.contains(&Field::References) {
            out.references = Some(character.references.clone());
        }
        if fields.contains(&Field::QueryCodes) {
            out.query_codes = Some(character.query_codes.clone());
        }
        if fields.contains(&Field::Readings) {
            out.readings = Some(character.readings.clone());
        }
        if fields.contains(&Field::Nanori) {
            out.nanori = Some(character.nanori.clone());
        }
        if fields.contains(&Field::Decomposition) {
            out.decomposition = character.decomposition.clone();
        }
        if fields.contains(&Field::Translations) {
            out.translations = Some(Self::translations(&character.translations, languages));
        }
        out
    }

    fn translations(translations: &Translations, languages: &[String]) -> Translations {
        if languages.len() == 0 {
            translations.clone()
        } else {
            Self::filtered_translations(translations, languages)
        }
    }

    fn filtered_translations(translations: &Translations, languages: &[String]) -> Translations {
        let out: Translations = translations
            .iter()
            .filter(|(k, _)| languages.contains(k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        out
    }

    fn all_fields(character: &Character, languages: &[String]) -> Self {
        Self {
            literal: character.literal.clone(),
            codepoints: Some(character.codepoints.clone()),
            radicals: Some(character.radicals.clone()),
            grade: character.grade,
            stroke_counts: Some(character.stroke_counts.clone()),
            variants: Some(character.variants.clone()),
            frequency: character.frequency,
            radical_names: Some(character.radical_names.clone()),
            jlpt: character.jlpt,
            references: Some(character.references.clone()),
            query_codes: Some(character.query_codes.clone()),
            readings: Some(character.readings.clone()),
            nanori: Some(character.nanori.clone()),
            decomposition: character.decomposition.clone(),
            translations: Some(Self::translations(&character.translations, languages)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Hash, FromFormField)]
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
