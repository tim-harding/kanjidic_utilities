#[macro_use]
extern crate rocket;

use cache::Cache;
use mongodb::bson::doc;
use rocket::{fairing::AdHoc, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
        .mount("/", routes![kanji, radicals])
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct KanjiResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<String>,
    kanji: Vec<CharacterResponse>,
}

#[get("/kanji?<literal>&<field>&<language>")]
async fn kanji(
    literal: Vec<String>,
    field: Vec<Field>,
    language: Vec<String>,
    cache: &State<Cache>,
) -> Result<Json<KanjiResponse>, &'static str> {
    let fields: HashSet<_> = field.into_iter().collect();
    let languages: HashSet<_> = language.into_iter().collect();
    let mut errors = vec![];
    let kanji: Vec<_> = literal
        .iter()
        .filter_map(|literal| match cache.kanji.get(literal) {
            Some(data) => Some(CharacterResponse::new(&data.character, &fields, &languages)),
            None => {
                errors.push(format!("Could not find kanji: {}", literal));
                None
            }
        })
        .collect();
    let response = KanjiResponse { errors, kanji };
    Ok(Json(response))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RadicalsResponse {
    pub errors: Vec<String>,
    pub valid_next: HashSet<String>,
    pub kanji: Vec<CharacterResponse>,
}

#[get("/radicals?<radical>&<field>&<language>")]
async fn radicals(
    radical: Vec<String>,
    field: Vec<Field>,
    language: Vec<String>,
    cache: &State<Cache>,
) -> Result<Json<RadicalsResponse>, &'static str> {
    let fields: HashSet<_> = field.into_iter().collect();
    let languages: HashSet<_> = language.into_iter().collect();
    let mut errors = vec![];
    let mut decomposition_sets = vec![];
    for radical in radical {
        match cache.radk.get(&radical) {
            Some(set) => decomposition_sets.push(set),
            None => {
                errors.push(format!("Could not find radical: {}", radical));
            }
        }
    }
    let literals: Vec<_> = match decomposition_sets.pop() {
        Some(set) => set
            .iter()
            .filter(|literal| decomposition_sets.iter().all(|&s| s.contains(*literal)))
            .collect(),
        None => vec![],
    };
    let mut errors = vec![];
    let mut decomposition_sets = vec![];
    let kanji: Vec<_> = literals
        .iter()
        .filter_map(|&literal| match cache.kanji.get(literal) {
            Some(data) => {
                if let Some(set) = &data.decomposition {
                    decomposition_sets.push(set);
                }
                Some(CharacterResponse::new(&data.character, &fields, &languages))
            }
            None => {
                errors.push(format!("Could not find kanji: {}", literal));
                None
            }
        })
        .collect();
    let valid_next: HashSet<_> = match decomposition_sets.pop() {
        Some(set) => set
            .clone()
            .into_iter()
            .filter(|radical| decomposition_sets.iter().all(|s| s.contains(radical)))
            .collect(),
        None => HashSet::default(),
    };
    let response = RadicalsResponse {
        errors,
        valid_next,
        kanji,
    };
    Ok(Json(response))
}
