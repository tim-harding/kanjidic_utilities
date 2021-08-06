use serde::{Deserialize, Serialize};

use crate::{Kunyomi, PinYin};

/// A particular reading or pronunciation of a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum Reading {
    /// The modern romanization of the Chinese reading.
    PinYin(PinYin),
    /// The romanized form of the Korean reading.
    KoreanRomanized(String),
    /// The Korean reading of the kanji in Hangul.
    KoreanHangul(String),
    /// The Vietnamese reading supplied by Minh Chau Pham.
    Vietnam(String),
    /// The onyomi reading of the kanji in katakana.
    Onyomi(String),
    /// The kunyomi reading of the kanji in hiragana or katakana.
    Kunyomi(Kunyomi),
}
