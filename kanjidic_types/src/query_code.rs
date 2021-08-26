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
#[serde(rename_all = "camelCase")]
pub struct Misclassification {
    /// The skip code of the misclassification
    pub skip: Skip,
    /// The kind of misclassification
    pub kind: MisclassificationKind,
}

/// A kind of kanji misclassification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MisclassificationKind {
    /// A mistake in the division of the kanji
    Position,
    /// A mistake in the number of strokes
    StrokeCount,
    /// Mistakes in both the division and the number of strokes
    StrokeAndPosition,
    /// Ambiguous stroke counts
    Ambiguous,
}
