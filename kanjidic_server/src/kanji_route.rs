use crate::{cache::Cache, character_response::CharacterResponse, field::Field};
use rocket::{serde::json::Json, State};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KanjiResponse<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<String>,
    kanji: Vec<CharacterResponse<'a>>,
}

#[get("/kanji/literals?<literal>&<field>&<language>")]
pub async fn kanji<'a>(
    literal: Vec<String>,
    field: Vec<Field>,
    language: Vec<String>,
    cache: &'a State<Cache>,
) -> Result<Json<KanjiResponse<'a>>, &'static str> {
    let fields: HashSet<_> = field.into_iter().collect();
    let languages: HashSet<_> = language.into_iter().collect();
    let mut errors = vec![];
    let kanji: Vec<_> = literal
        .iter()
        .filter_map(|literal| match cache.kanji.get(literal) {
            Some(character) => Some(CharacterResponse::new(&character, &fields, &languages)),
            None => {
                errors.push(format!("Could not find kanji: {}", literal));
                None
            }
        })
        .collect();
    let response = KanjiResponse { errors, kanji };
    Ok(Json(response))
}
