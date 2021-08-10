use serde::{Deserialize, Serialize};

use crate::{Reading, Translation};

/// Information about a particular meaning of a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Meaning {
    /// Different ways the kanji can be read.
    pub readings: Vec<Reading>,
    /// Translations of the kanji into different languages.
    pub translations: Vec<Translation>,
}
