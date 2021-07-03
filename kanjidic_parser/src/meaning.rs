use crate::{reading::Reading, translation::Translation};

/// Information about a particular meaning of a kanji.
pub struct Meaning<'a> {
    /// Different ways the kanji can be read.
    pub readings: Vec<Reading<'a>>,
    
    /// Translations of the kanji into different languages.
    pub translations: Vec<Translation<'a>>,
    
    /// Japanese readings associated with names.
    pub nanori: Vec<&'a str>,
}
