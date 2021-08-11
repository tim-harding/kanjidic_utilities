use kanjidic_types::{
    Codepoint, Grade, QueryCode, Radical, Reading, Reference, StrokeCount, Translations, Variant,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CharacterResponse {
    pub literal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codepoints: Option<Vec<Codepoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radicals: Option<Vec<Radical>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<Grade>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_counts: Option<StrokeCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<Variant>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radical_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jlpt: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<Reference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_codes: Option<Vec<QueryCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readings: Option<Vec<Reading>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<Translations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanori: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decomposition: Option<Vec<String>>,
}
