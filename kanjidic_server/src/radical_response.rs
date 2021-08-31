use std::collections::HashSet;
use crate::cache::Radk;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Hash, FromFormField)]
pub enum Field {
    Strokes,
    Kanji,
}

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
