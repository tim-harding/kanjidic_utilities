use serde::{Deserialize, Serialize};

use crate::LanguageCode;

// Todo: Identify suffixes and prefixes
/// A translation of a kanji meaning into another language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Translation {
    /// The word in the target language.
    pub text: String,
    /// The language being translated into.
    pub language: LanguageCode,
}
