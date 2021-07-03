use iso639_1::Iso639_1;

/// A translation of a kanji meaning into another language.
pub struct Translation<'a> {
    /// The word in the target language.
    pub text: &'a str,
    
    /// The language being translated into.
    pub language: Iso639_1,
}
