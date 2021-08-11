use std::collections::HashSet;

use kanjidic_types::{
    Character, Codepoint, Grade, QueryCode, Radical, Reading, Reference, StrokeCount, Translations,
    Variant,
};
use serde::{Deserialize, Serialize};

use crate::field::Field;

type Languages = HashSet<String>;
type Fields = HashSet<Field>;

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

impl CharacterResponse {
    pub fn new(character: Character, fields: &Fields, languages: &Languages) -> Self {
        if fields.len() == 0 {
            Self::all_fields(character, languages)
        } else {
            Self::filtered_fields(character, fields, languages)
        }
    }

    fn filtered_fields(character: Character, fields: &Fields, languages: &Languages) -> Self {
        let mut out = CharacterResponse::default();
        out.literal = character.literal;
        if fields.contains(&Field::Codepoints) {
            out.codepoints = Some(character.codepoints);
        }
        if fields.contains(&Field::Radicals) {
            out.radicals = Some(character.radicals);
        }
        if fields.contains(&Field::Grade) {
            out.grade = character.grade;
        }
        if fields.contains(&Field::StrokeCounts) {
            out.stroke_counts = Some(character.stroke_counts);
        }
        if fields.contains(&Field::Variants) {
            out.variants = Some(character.variants);
        }
        if fields.contains(&Field::Frequency) {
            out.frequency = character.frequency;
        }
        if fields.contains(&Field::RadicalNames) {
            out.radical_names = Some(character.radical_names);
        }
        if fields.contains(&Field::Jlpt) {
            out.jlpt = character.jlpt;
        }
        if fields.contains(&Field::References) {
            out.references = Some(character.references);
        }
        if fields.contains(&Field::QueryCodes) {
            out.query_codes = Some(character.query_codes);
        }
        if fields.contains(&Field::Readings) {
            out.readings = Some(character.readings);
        }
        if fields.contains(&Field::Nanori) {
            out.nanori = Some(character.nanori);
        }
        if fields.contains(&Field::Decomposition) {
            out.decomposition = character.decomposition;
        }
        if fields.contains(&Field::Translations) {
            out.translations = Some(Self::translations(character.translations, languages));
        }
        out
    }

    fn translations(translations: Translations, languages: &Languages) -> Translations {
        if languages.len() == 0 {
            translations
        } else {
            Self::filtered_translations(translations, languages)
        }
    }

    fn filtered_translations(translations: Translations, languages: &Languages) -> Translations {
        let out: Translations = translations
            .into_iter()
            .filter(|(k, _)| languages.contains(k))
            .collect();
        out
    }

    fn all_fields(character: Character, languages: &Languages) -> Self {
        Self {
            literal: character.literal,
            codepoints: Some(character.codepoints),
            radicals: Some(character.radicals),
            grade: character.grade,
            stroke_counts: Some(character.stroke_counts),
            variants: Some(character.variants),
            frequency: character.frequency,
            radical_names: Some(character.radical_names),
            jlpt: character.jlpt,
            references: Some(character.references),
            query_codes: Some(character.query_codes),
            readings: Some(character.readings),
            nanori: Some(character.nanori),
            decomposition: character.decomposition,
            translations: Some(Self::translations(character.translations, languages)),
        }
    }
}
