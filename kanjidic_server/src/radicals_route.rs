use serde::Serialize;
use std::collections::HashSet;
use rocket::{serde::json::Json, State};
use crate::{cache::Cache, character_response::CharacterResponse, field::Field};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RadicalsResponse<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    pub valid_next: HashSet<&'a str>,
    pub kanji: Vec<CharacterResponse<'a>>,
}

#[get("/radicals?<radical>&<field>&<language>")]
pub async fn radicals<'a>(
    radical: Vec<String>,
    field: Vec<Field>,
    language: Vec<String>,
    cache: &'a State<Cache>,
) -> Result<Json<RadicalsResponse<'a>>, &'static str> {
    let fields: HashSet<_> = field.into_iter().collect();
    let languages: HashSet<_> = language.into_iter().collect();
    let mut errors = vec![];
    let mut decomposition_sets = vec![];
    for radical in radical.iter() {
        match cache.radk.get(radical) {
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
    let mut valid_next = HashSet::default();
    let kanji: Vec<_> = literals
        .iter()
        .filter_map(|&literal| match cache.kanji.get(literal) {
            Some(character) => {
                if let Some(decomposition) = &character.decomposition {
                    valid_next.extend(decomposition.iter().map(|s| s.as_str()));
                }
                Some(CharacterResponse::new(&character, &fields, &languages))
            }
            None => {
                errors.push(format!("Could not find kanji: {}", literal));
                None
            }
        })
        .collect();
    for radical in radical.iter() {
        let _ = valid_next.remove(radical.as_str());
    }
    let response = RadicalsResponse {
        errors,
        valid_next,
        kanji,
    };
    Ok(Json(response))
}