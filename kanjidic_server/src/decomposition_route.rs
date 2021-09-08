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
pub async fn decomposition<'a>(
    radicals: String,
    field: Vec<Field>,
    language: Vec<String>,
    page: Option<u16>,
    limit: Option<u16>,
    cache: &'a State<Cache>,
) -> Result<Json<RadicalsResponse<'a>>, &'static str> {
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
    if radicals.len() == 0 {
        let valid_next: HashSet<_> = cache.radk.keys().map(|&k| k).collect();
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
            .filter(predicate)
            .filter_map(|kanji_literal| {
                let kanji_contains_all_radicals = decomposition_sets
                    .iter()
                    .all(|&set| set.kanji.contains(kanji_literal));
                if kanji_contains_all_radicals {
                    match cache.kanji.get(kanji_literal) {
                        Some(kanji) => {
                            valid_next.extend(kanji.decomposition.iter());
                            Some(CharacterResponse::new(&kanji, &field, &language))
                        }
                        None => {
                            errors.push(format!("Could not find kanji: {}", kanji_literal));
                            None
                        }
                    }
                } else {
                    None
                }
            })
            // Can't do skip/take here because we need the side
            // effects from the closure.
            .collect(),
        None => vec![],
    };
    let kanji: Vec<_> = kanji.into_iter().skip(page * limit).take(limit).collect();
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
