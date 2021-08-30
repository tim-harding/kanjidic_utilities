use crate::{
    cache::Cache, character_response::CharacterResponse, field::Field, shared::string_to_char,
};
use rocket::{serde::json::Json, State};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RadicalsResponse<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    pub valid_next: HashSet<char>,
    pub kanji: Vec<CharacterResponse<'a>>,
}

// Todo: limit results
// Todo: cache zero and one radical combinations

#[get("/kanji/decomposition?<radical>&<field>&<language>")]
pub async fn decomposition<'a>(
    radical: Vec<String>,
    field: Vec<Field>,
    language: Vec<String>,
    cache: &'a State<Cache>,
) -> Result<Json<RadicalsResponse<'a>>, &'static str> {
    let mut errors = vec![];
    let radicals: Vec<_> = radical
        .into_iter()
        .filter_map(|s| {
            let radical = string_to_char(&s);
            if radical.is_none() {
                errors.push(format!("Radicals should be one unicode codepoint: {}", s));
            }
            radical
        })
        .collect();
    let fields: HashSet<_> = field.into_iter().collect();
    let languages: HashSet<_> = language.into_iter().collect();
    let mut decomposition_sets = vec![];
    for radical in radicals.iter() {
        match cache.radk.get(&radical) {
            Some(set) => decomposition_sets.push(set),
            None => {
                errors.push(format!("Could not find radical: {}", radical));
            }
        }
    }
    let literals: Vec<_> = match decomposition_sets.pop() {
        Some(set) => set
            .kanji
            .iter()
            .filter(|literal| {
                decomposition_sets
                    .iter()
                    .all(|&s| s.kanji.contains(*literal))
            })
            .collect(),
        None => vec![],
    };
    let mut valid_next: HashSet<char> = HashSet::default();
    let kanji: Vec<_> = literals
        .iter()
        .filter_map(|&literal| match cache.kanji.get(literal) {
            Some(character) => {
                valid_next.extend(character.decomposition.iter());
                Some(CharacterResponse::new(&character, &fields, &languages))
            }
            None => {
                errors.push(format!("Could not find kanji: {}", literal));
                None
            }
        })
        .collect();
    for radical in radicals.iter() {
        let _ = valid_next.remove(radical);
    }
    let response = RadicalsResponse {
        errors,
        valid_next,
        kanji,
    };
    Ok(Json(response))
}
