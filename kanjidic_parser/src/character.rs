use crate::{codepoint::Codepoint, grade::Grade, radical::Radical, stroke_count::StrokeCount, variant::Variant};

/// Information about a kanji.
pub struct Character<'a> {
    /// The character itself.
    pub literal: &'a str,
    
    /// Alternate encodings for the character.
    pub codepoints: Vec<Codepoint>,
    
    /// Alternate classifications for the character by radical.
    pub radicals: Vec<Radical>,
    
    /// The kanji grade level.
    pub grade: Option<Grade>,
    
    /// The stroke count of the character.
    pub stroke_count: StrokeCount,
    
    /// Cross-references to other characters or alternative indexings.
    pub variants: Vec<Variant>,
    
    /// A ranking of how often the character appears in newspapers.
    pub frequency: Option<u16>,

    /// The kanji's name as a radical if it is one.
    pub radical_names: Vec<&'a str>,
    
    /// Old JLPT level of the kanji. Based on pre-2010 test levels 
    /// that go up to four, not five.
    pub jlpt: Option<u8>,
}