use crate::{cache::Cache, character_response::CharacterResponse, field::Field};
use rocket::{serde::json::Json, State};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadicalsResponse<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    pub valid_next: HashSet<char>,
    pub kanji: Vec<CharacterResponse<'a>>,
}

#[get("/kanji/decomposition/<radicals>?<field>&<language>&<page>&<limit>")]
pub async fn decomposition(
    radicals: String,
    field: Vec<Field>,
    language: Vec<String>,
    page: Option<u16>,
    limit: Option<u16>,
    cache: &State<Cache>,
) -> Result<Json<RadicalsResponse>, &'static str> {
    let limit = match limit {
        Some(limit) => std::cmp::min(limit, 16),
        None => 16,
    } as usize;
    let page = page.unwrap_or(0);
    let mut errors = vec![];
    let field: HashSet<_> = field.into_iter().collect();
    let language: HashSet<_> = language.into_iter().collect();
    if radicals.is_empty() {
        let valid_next: HashSet<_> = cache.radk.keys().copied().collect();
        return Ok(Json(RadicalsResponse {
            errors,
            valid_next,
            kanji: vec![],
        }));
    }
    let (decomposition_sets, first_decomposition_set) = {
        let mut decomposition_sets: Vec<_> = radicals
            .chars()
            .filter_map(|radical| match cache.radk.get(&radical) {
                Some(set) => Some(set),
                None => {
                    errors.push(format!("Could not find radical: {}", radical));
                    None
                }
            })
            .collect();
        let first = decomposition_sets.pop();
        (decomposition_sets, first)
    };
    let mut valid_next: HashSet<char> = HashSet::default();
    let kanji: Vec<_> = match first_decomposition_set {
        Some(first) => first
            .kanji
            .iter()
            .filter(|kanji_literal| {
                decomposition_sets
                    .iter()
                    .all(|&set| set.kanji.contains(kanji_literal))
            })
            .filter_map(|kanji_literal| match cache.kanji.get(kanji_literal) {
                Some(kanji) => {
                    valid_next.extend(kanji.decomposition.iter());
                    Some(CharacterResponse::new(kanji, &field, &language))
                }
                None => {
                    errors.push(format!("Could not find kanji: {}", kanji_literal));
                    None
                }
            })
            // Can't do skip/take here because we need the side
            // effects from the closure.
            .collect(),
        None => vec![],
    };
    let kanji: Vec<_> = kanji
        .into_iter()
        .skip(page as usize * limit)
        .take(limit)
        .collect();
    for radical in radicals.chars() {
        let _ = valid_next.remove(&radical);
    }
    let response = RadicalsResponse {
        errors,
        valid_next,
        kanji,
    };
    Ok(Json(response))
}
