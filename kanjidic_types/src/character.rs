use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Codepoint, Grade, QueryCode, Radical, Reading, Reference, StrokeCount, Variant};

pub type Translations = HashMap<String, Vec<String>>;

/// Information about a kanji.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    /// The character itself.
    pub literal: String,
    /// Alternate encodings for the character.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub codepoints: Vec<Codepoint>,
    /// Alternate classifications for the character by radical.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub radicals: Vec<Radical>,
    /// The kanji grade level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<Grade>,
    /// The stroke count of the character.
    pub stroke_counts: StrokeCount,
    /// Cross-references to other characters or alternative indexings.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<Variant>,
    /// A ranking of how often the character appears in newspapers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<u16>,
    /// The kanji's name as a radical if it is one.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub radical_names: Vec<String>,
    /// Old JLPT level of the kanji. Based on pre-2010 test levels
    /// that go up to four, not five.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jlpt: Option<u8>,
    /// Indexes into dictionaries and other instructional books
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<Reference>,
    /// Codes used to identify the kanji
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub query_codes: Vec<QueryCode>,
    /// Different ways the kanji can be read.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub readings: Vec<Reading>,
    /// Translations of the kanji into different languages.
    pub translations: Translations,
    /// Japanese readings associated with names.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nanori: Vec<String>,
    /// The constituent radicals in the kanji
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub decomposition: Vec<String>,
}
