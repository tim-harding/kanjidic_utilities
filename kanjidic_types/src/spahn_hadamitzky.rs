use serde::{Deserialize, Serialize};

// They are in the form nxnn.n,
// e.g.  3k11.2, where the  kanji has 3 strokes in the
// identifying radical, it is radical "k" in the SH
// classification system, there are 11 other strokes, and it is
// the 2nd kanji in the 3k11 sequence.

/// Descriptor code for The Kanji Dictionary.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ShDesc {
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
