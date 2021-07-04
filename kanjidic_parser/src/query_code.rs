use crate::{
    de_roo::DeRoo, four_corner::FourCorner, skip::Skip, spahn_hadamitzky::SpahnHadamitzkyDescriptor,
};

/// Information relating to a kanji that can be
/// used for identification and lookup.
pub enum QueryCode {
    /// The Halpern SKIP code
    Skip(Skip),

    /// Desrcriptor codes from The Kanji Dictionary
    SpahnHadamitzky(SpahnHadamitzkyDescriptor),

    /// The Four Corner code
    FourCorner(FourCorner),

    /// Father Joseph De Roo's code system
    DeRoo(DeRoo),

    /// A possible misclassification of the kanji
    Misclassification(Misclassification),
}

/// A possible misclassification of the kanji
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
