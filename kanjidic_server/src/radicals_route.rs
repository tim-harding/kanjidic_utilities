use std::collections::{HashMap, HashSet};

use crate::{cache::{Cache, Radk}, shared::string_to_char};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RadicalResponse<'a> {
    literal: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    strokes: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kanji: Option<&'a HashSet<char>>,
}

impl<'a, 'b> RadicalResponse<'a> {
    pub fn new(radk: &'a Radk, fields: &'b [Field]) -> Self {
        let mut out = Self {
            literal: radk.radical,
            strokes: None,
            kanji: None,
        };
        for field in fields {
            match field {
                Field::Strokes => out.strokes = Some(radk.stroke),
                Field::Kanji => out.kanji = Some(&radk.kanji),
            }
        }
        out
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Hash, FromFormField)]
pub enum Field {
    Strokes,
    Kanji,
}

#[get("/radicals/literals?<literal>&<field>")]
pub async fn radicals_some<'a>(
    literal: Vec<String>,
    field: Vec<Field>,
    cache: &'a State<Cache>,
) -> Result<Json<Vec<RadicalResponse<'a>>>, &'static str> {
    let mut errors = vec![];
    let literals: Vec<_> = literal
        .into_iter()
        .filter_map(|s| {
            let literal = string_to_char(&s);
            if literal.is_none() {
                errors.push(format!("Literals should be one unicode codepoint: {}", s));
            }
            literal
        })
        .collect();
    let radicals: Vec<_> = literals
        .into_iter()
        .filter_map(|literal| {
            let response = cache
                .radk
                .get(&literal)
                .map(|radical| RadicalResponse::new(&radical, &field));
            if response.is_none() {
                errors.push(format!("Could not find literal: {}", literal))
            }
            response
        })
        .collect();
    Ok(Json(radicals))
}

// Todo: cache this
type AllRadicals = HashMap<u8, Vec<char>>;

#[get("/radicals/all")]
pub async fn radicals_all<'a>(
    cache: &'a State<Cache>,
) -> Result<Json<AllRadicals>, &'static str> {
    let mut out = AllRadicals::default();
    for radical in cache
        .radk
        .values() {
        match out.entry(radical.stroke) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(radical.radical);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![radical.radical]);
            }
        }
    }
    Ok(Json(out))
}
