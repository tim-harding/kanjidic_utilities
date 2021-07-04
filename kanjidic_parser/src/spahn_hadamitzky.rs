/// Descriptor code for The Kanji Dictionary.
pub struct SpahnHadamitzkyDescriptor {
    /// Number of strokes in the identifying radical.
    pub radical_strokes: u8,

    /// The letter for the radical in the identification system.
    pub radical: char,

    /// The number of strokes not included in the radical.
    pub other_strokes: u8,

    /// The position of the kanji in the sequence described
    /// by the other descriptor parts.
    pub sequence: u8,
}
