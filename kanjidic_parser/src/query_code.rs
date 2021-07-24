use std::convert::TryFrom;

use crate::{
    de_roo::{DeRoo, DeRooError},
    four_corner::{FourCorner, FourCornerError},
    skip::{Skip, SkipError},
    spahn_hadamitzky::{ShError, SpahnHadamitzkyDescriptor},
};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum QueryCodeError {
    #[error("Missing qc_type attribute")]
    MissingType,
    #[error("Unknown qc_type attribute")]
    UnknownType,
    #[error("Error while parsing skip code")]
    Skip(#[from] SkipError),
    #[error("Error while parsing Spahn Hadamitzky descriptor")]
    SpahnHadamitzky(#[from] ShError),
    #[error("Error while parsing four corner code")]
    FourCorner(#[from] FourCornerError),
    #[error("Error while parsing de roo code")]
    DeRoo(#[from] DeRooError),
    #[error("Unrecognized skip_misclass value")]
    UnknownMisclassification,
}

/// Information relating to a kanji that can be
/// used for identification and lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a, 'input> TryFrom<Node<'a, 'input>> for QueryCode {
    type Error = QueryCodeError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let qc_type = node
            .attribute("qc_type")
            .ok_or(QueryCodeError::MissingType)?;
        match qc_type {
            "skip" => {
                if let Some(misclass_kind) = node.attribute("skip_misclass") {
                    Ok(QueryCode::Misclassification(match misclass_kind {
                        "posn" => Ok(Misclassification::Position(Skip::try_from(node)?)),
                        "stroke_count" => Ok(Misclassification::StrokeCount(Skip::try_from(node)?)),
                        "stroke_and_posn" => {
                            Ok(Misclassification::StrokeAndPosition(Skip::try_from(node)?))
                        }
                        "stroke_diff" => Ok(Misclassification::Ambiguous(Skip::try_from(node)?)),
                        _ => Err(QueryCodeError::UnknownMisclassification),
                    }?))
                } else {
                    Ok(QueryCode::Skip(Skip::try_from(node)?))
                }
            }
            "sh_desc" => Ok(QueryCode::SpahnHadamitzky(
                SpahnHadamitzkyDescriptor::try_from(node)?,
            )),
            "four_corner" => Ok(QueryCode::FourCorner(FourCorner::try_from(node)?)),
            "deroo" => Ok(QueryCode::DeRoo(DeRoo::try_from(node)?)),
            _ => Err(QueryCodeError::UnknownType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        skip::{SkipSolid, SolidSubpattern},
        test_shared::DOC,
    };

    #[test]
    fn query_code() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("q_code"))
            .unwrap();
        let query_code = QueryCode::try_from(node);
        assert_eq!(
            query_code,
            Ok(QueryCode::Skip(Skip::Solid(SkipSolid {
                total_stroke_count: 7,
                solid_subpattern: SolidSubpattern::TopLine,
            })))
        )
    }
}
