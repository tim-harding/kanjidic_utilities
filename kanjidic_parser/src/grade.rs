/// The grade level in which the kanji is learned.
pub enum Grade {
    /// A Kyouiku kanji learned in grades 1-6. 
    Kyouiku(u8),
    
    /// A remaining Jouyou kanji to be learned in junior hi-school.
    Jouyou,
    
    /// A Jinmeiyou kanji for use in names that is approved
    /// for use in family name registers and other official documents.
    Jinmeiyou,
    
    /// A Jinmeiyou kanji that is a variant of a Jouyou kanji.
    JinmeiyouJouyouVariant,
}