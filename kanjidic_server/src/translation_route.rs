use crate::{cache::Cache, character_response::CharacterResponse, field::Field};
use rocket::{serde::json::Json, State};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationResponse<'a> {
    kanji: Vec<CharacterResponse<'a>>,
}

// Todo: Assert that `translation` is composed of ascii letters

#[get("/kanji/translation/<translation>?<field>&<language>&<page>&<limit>")]
pub async fn translation(
    translation: String,
    field: Vec<Field>,
    language: Vec<String>,
    page: Option<u16>,
    limit: Option<u16>,
    cache: &State<Cache>,
) -> Json<TranslationResponse<'_>> {
    let translation = translation.to_lowercase();
    let limit = match limit {
        Some(limit) => std::cmp::min(limit, 16),
        None => 16,
    } as usize;
    let page = page.unwrap_or(0);
    let field: HashSet<_> = field.into_iter().collect();
    let language: HashSet<_> = language.into_iter().collect();
    let kanji = match cache.translations.get(&translation) {
        Some(literals) => literals
            .iter()
            .filter_map(|literal| {
                cache
                    .kanji
                    .get(literal)
                    .map(|character| CharacterResponse::new(character, &field, &language))
            })
            .skip(page as usize * limit)
            .take(limit)
            .collect(),
        None => vec![],
    };
    Json(TranslationResponse { kanji })
}
