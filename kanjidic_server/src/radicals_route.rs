use crate::{
    cache::Cache,
    radical_response::{Field, RadicalResponse},
    shared::string_to_char,
};
use rocket::{serde::json::Json, State};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RadicalSomeResponse<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<String>,
    radicals: Vec<RadicalResponse<'a>>,
}

#[get("/radicals/literals?<literal>&<field>")]
pub async fn radicals_some<'a>(
    literal: Vec<String>,
    field: Vec<Field>,
    cache: &'a State<Cache>,
) -> Result<Json<RadicalSomeResponse<'a>>, &'static str> {
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
    let response = RadicalSomeResponse { radicals, errors };
    Ok(Json(response))
}

#[derive(Debug, Clone, Serialize, Hash)]
pub struct AllRadical {
    strokes: u8,
    literals: Vec<char>,
}

impl PartialEq for AllRadical {
    fn eq(&self, other: &Self) -> bool {
        self.strokes == other.strokes
    }
}

impl Eq for AllRadical {}

impl PartialOrd for AllRadical {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.strokes.partial_cmp(&other.strokes)
    }
}

impl Ord for AllRadical {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.strokes.cmp(&other.strokes)
    }
}

#[get("/radicals/all")]
pub async fn radicals_all<'a>(
    cache: &'a State<Cache>,
) -> Result<Json<Vec<AllRadical>>, &'static str> {
    let mut collect: HashMap<u8, Vec<char>> = HashMap::default();
    for radical in cache.radk.values() {
        match collect.entry(radical.stroke) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(radical.radical);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![radical.radical]);
            }
        }
    }
    let mut out: Vec<_> = collect
        .into_iter()
        .map(|(strokes, radicals)| AllRadical {
            strokes,
            literals: radicals,
        })
        .collect();
    out.sort();
    Ok(Json(out))
}
