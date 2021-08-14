use std::collections::HashSet;

use crate::cache::{Cache, Radk};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RadicalResponse<'a> {
    literal: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    strokes: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kanji: Option<&'a HashSet<String>>,
}

impl<'a, 'b> RadicalResponse<'a> {
    pub fn new(radk: &'a Radk, fields: &'b [Field]) -> Self {
        let mut out = Self {
            literal: &radk.radical,
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
    println!("{:?}", field);
    let radicals: Vec<_> = literal
        .iter()
        .filter_map(|literal| {
            cache
                .radk
                .get(literal)
                .map(|radical| RadicalResponse::new(&radical, &field))
        })
        .collect();
    Ok(Json(radicals))
}

#[get("/radicals/all")]
pub async fn radicals_all<'a>(
    cache: &'a State<Cache>,
) -> Result<Json<Vec<RadicalResponse<'a>>>, &'static str> {
    let radicals: Vec<_> = cache
        .radk
        .iter()
        .map(|(_k, radical)| RadicalResponse::new(&radical, &[Field::Strokes]))
        .collect();
    Ok(Json(radicals))
}
