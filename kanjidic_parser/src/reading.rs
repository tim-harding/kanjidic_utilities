use crate::{kunyomi::Kunyomi, pin_yin::PinYin};

/// A particular reading or pronunciation of a kanji.
pub enum Reading<'a> {
    /// The modern romanization of the Chinese reading.
    PinYin(PinYin<'a>),
    
    /// The romanized form of the Korean reading.
    KoreanRomanized(&'a str),
    
    /// The Korean reading of the kanji in Hangul.
    KoreanHangul(&'a str),
    
    /// The Vietnamese reading supplied by Minh Chau Pham.
    Vietnam(&'a str),
    
    /// The onyomi reading of the kanji in katakana.
    Onyomi(&'a str),
    
    /// The kunyomi reading of the kanji in hiragana or katakana.
    Kunyomi(Kunyomi<'a>),
}
