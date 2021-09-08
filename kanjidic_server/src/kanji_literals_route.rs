use crate::{
    cache::Cache, character_response::CharacterResponse, field::Field, shared::string_to_char,
};
use rocket::{serde::json::Json, State};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KanjiResponse<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<String>,
    kanji: Vec<CharacterResponse<'a>>,
}

#[get("/kanji/literals/<literal>?<field>&<language>&<limit>&<page>")]
pub async fn kanji<'a>(
    literal: String,
    field: Vec<Field>,
    language: Vec<String>,
    limit: Option<u16>,
    page: Option<u16>,
    cache: &'a State<Cache>,
) -> Result<Json<KanjiResponse<'a>>, &'static str> {
    let limit = match limit {
        Some(limit) => std::cmp::min(limit, 16),
        None => 16,
    } as usize;
    let page = match page {
        Some(page) => page,
        None => 0,
    } as usize;
    let mut errors = vec![];
    let field: HashSet<_> = field.into_iter().collect();
    let language: HashSet<_> = language.into_iter().collect();
    let kanji: Vec<_> = literal
        .chars()
        .filter_map(|s| match cache.kanji.get(&s) {
            Some(character) => Some(CharacterResponse::new(&character, &field, &language)),
            None => {
                errors.push(format!("Could not find kanji: {}", literal));
                None
            }
        })
        .skip(page * limit)
        .take(limit)
        .collect();
    let response = KanjiResponse { errors, kanji };
    Ok(Json(response))
}
