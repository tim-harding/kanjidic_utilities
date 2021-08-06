use serde::{Deserialize, Serialize};

use crate::{DeRoo, FourCorner, ShDesc, Skip};

/// Information relating to a kanji that can be
/// used for identification and lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum QueryCode {
    /// The Halpern SKIP code
    Skip(Skip),
    /// Desrcriptor codes from The Kanji Dictionary
    SpahnHadamitzky(ShDesc),
    /// The Four Corner code
    FourCorner(FourCorner),
    /// Father Joseph De Roo's code system
    DeRoo(DeRoo),
    /// A possible misclassification of the kanji
    Misclassification(Misclassification),
}

/// A possible misclassification of the kanji
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum Misclassification {
    /// A mistake in the division of the kanji
    Position(Skip),
    /// A mistake in the number of strokes
    StrokeCount(Skip),
    /// Mistakes in both the division and the number of strokes
    StrokeAndPosition(Skip),
    /// Ambiguous stroke counts
    Ambiguous(Skip),
}
