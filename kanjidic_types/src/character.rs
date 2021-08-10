use serde::{Deserialize, Serialize};

use crate::{Codepoint, Grade, Meaning, QueryCode, Radical, Reference, StrokeCount, Variant};

/// Information about a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Character {
    /// The character itself.
    pub literal: String,
    /// Alternate encodings for the character.
    pub codepoints: Vec<Codepoint>,
    /// Alternate classifications for the character by radical.
    pub radicals: Vec<Radical>,
    /// The kanji grade level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<Grade>,
    /// The stroke count of the character.
    pub stroke_counts: StrokeCount,
    /// Cross-references to other characters or alternative indexings.
    pub variants: Vec<Variant>,
    /// A ranking of how often the character appears in newspapers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<u16>,
    /// The kanji's name as a radical if it is one.
    pub radical_names: Vec<String>,
    /// Old JLPT level of the kanji. Based on pre-2010 test levels
    /// that go up to four, not five.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jlpt: Option<u8>,
    /// Indexes into dictionaries and other instructional books
    pub references: Vec<Reference>,
    /// Codes used to identify the kanji
    pub query_codes: Vec<QueryCode>,
    /// Different meanings of the kanji.
    pub meanings: Vec<Meaning>,
    /// Japanese readings associated with names.
    pub nanori: Vec<String>,
    /// The constituent radicals in the kanji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decomposition: Option<Vec<String>>,
}
