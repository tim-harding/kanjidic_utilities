use kanjidic_types::{
    Character, Codepoint, Grade, QueryCode, Radical, Reading, Reference, StrokeCount, Translations,
    Variant,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

use crate::field::Field;

// Todo: also skip parsing empty arrays?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct CharacterResponse<'a> {
    pub literal: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codepoints: Option<&'a [Codepoint]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radicals: Option<&'a [Radical]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<Grade>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_counts: Option<&'a StrokeCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<&'a [Variant]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radical_names: Option<&'a [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jlpt: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<&'a [Reference]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_codes: Option<&'a [QueryCode]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readings: Option<&'a [Reading]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<TranslationsResponse<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanori: Option<&'a [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decomposition: Option<&'a [String]>,
}

type FilteredTranslations<'a> = HashMap<&'a str, &'a [String]>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TranslationsResponse<'a> {
    All(&'a Translations),
    Some(FilteredTranslations<'a>),
}

type Languages = HashSet<String>;
type Fields = HashSet<Field>;

impl<'a, 'b> CharacterResponse<'a> {
    pub fn new(character: &'a Character, fields: &'b Fields, languages: &'b Languages) -> Self {
        if fields.contains(&Field::All) {
            Self::all_fields(character, languages)
        } else {
            Self::filtered_fields(character, fields, languages)
        }
    }

    // Todo: iterate and match on fields array,
    // don't use a hashset here

    fn filtered_fields(
        character: &'a Character,
        fields: &'b Fields,
        languages: &'b Languages,
    ) -> Self {
        let mut out = CharacterResponse::default();
        out.literal = &character.literal;
        if fields.contains(&Field::Codepoints) {
            out.codepoints = Some(&character.codepoints);
        }
        if fields.contains(&Field::Radicals) {
            out.radicals = Some(&character.radicals);
        }
        if fields.contains(&Field::Grade) {
            out.grade = character.grade;
        }
        if fields.contains(&Field::StrokeCounts) {
            out.stroke_counts = Some(&character.stroke_counts);
        }
        if fields.contains(&Field::Variants) {
            out.variants = Some(&character.variants);
        }
        if fields.contains(&Field::Frequency) {
            out.frequency = character.frequency;
        }
        if fields.contains(&Field::RadicalNames) {
            out.radical_names = Some(&character.radical_names);
        }
        if fields.contains(&Field::Jlpt) {
            out.jlpt = character.jlpt;
        }
        if fields.contains(&Field::References) {
            out.references = Some(&character.references);
        }
        if fields.contains(&Field::QueryCodes) {
            out.query_codes = Some(&character.query_codes);
        }
        if fields.contains(&Field::Readings) {
            out.readings = Some(&character.readings);
        }
        if fields.contains(&Field::Nanori) {
            out.nanori = Some(&character.nanori);
        }
        if fields.contains(&Field::Decomposition) {
            out.decomposition = Some(&character.decomposition)
        }
        if fields.contains(&Field::Translations) {
            out.translations = Some(Self::translations(&character.translations, languages));
        }
        out
    }

    fn translations(
        translations: &'a Translations,
        languages: &'b Languages,
    ) -> TranslationsResponse<'a> {
        if languages.len() == 0 {
            TranslationsResponse::All(&translations)
        } else {
            TranslationsResponse::Some(Self::filtered_translations(translations, languages))
        }
    }

    fn filtered_translations(
        translations: &'a Translations,
        languages: &'b Languages,
    ) -> FilteredTranslations<'a> {
        let out: FilteredTranslations = translations
            .iter()
            .filter(|(k, _)| languages.contains(*k))
            .map(|(k, v)| (k.as_str(), v.as_slice()))
            .collect();
        out
    }

    fn all_fields(character: &'a Character, languages: &'b Languages) -> Self {
        Self {
            literal: &character.literal,
            codepoints: Some(&character.codepoints),
            radicals: Some(&character.radicals),
            grade: character.grade,
            stroke_counts: Some(&character.stroke_counts),
            variants: Some(&character.variants),
            frequency: character.frequency,
            radical_names: Some(&character.radical_names),
            jlpt: character.jlpt,
            references: Some(&character.references),
            query_codes: Some(&character.query_codes),
            readings: Some(&character.readings),
            nanori: Some(&character.nanori),
            decomposition: Some(&character.decomposition),
            translations: Some(Self::translations(&character.translations, languages)),
        }
    }
}
