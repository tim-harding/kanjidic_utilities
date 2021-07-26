use std::convert::TryFrom;

use crate::{
    de_roo::{DeRoo, DeRooError},
    four_corner::{FourCorner, FourCornerError},
    pos_error::PosError,
    shared::{attr, SharedError},
    skip::{Skip, SkipError},
    spahn_hadamitzky::{ShDesc, ShError},
};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum QueryCodeError {
    #[error("(Query code) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Query code) Unknown qc_type attribute: {0}")]
    UnknownType(PosError),
    #[error("(Query code) Skip code: {0}")]
    Skip(#[from] SkipError),
    #[error("(Query code) Spahn Hadamitzky descriptor: {0}")]
    SpahnHadamitzky(#[from] ShError),
    #[error("(Query code) Four corner code: {0}")]
    FourCorner(#[from] FourCornerError),
    #[error("(Query code) De Roo code: {0}")]
    DeRoo(#[from] DeRooError),
    #[error("(Query code) Unrecognized skip_misclass value: {0}")]
    UnknownMisclassification(PosError),
}

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

impl<'a, 'input> TryFrom<Node<'a, 'input>> for QueryCode {
    type Error = QueryCodeError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let qc_type = attr(node, "qc_type")?;
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
                        _ => Err(QueryCodeError::UnknownMisclassification(PosError::from(
                            node,
                        ))),
                    }?))
                } else {
                    Ok(QueryCode::Skip(Skip::try_from(node)?))
                }
            }
            "sh_desc" => Ok(QueryCode::SpahnHadamitzky(ShDesc::try_from(node)?)),
            "four_corner" => Ok(QueryCode::FourCorner(FourCorner::try_from(node)?)),
            "deroo" => Ok(QueryCode::DeRoo(DeRoo::try_from(node)?)),
            _ => Err(QueryCodeError::UnknownType(PosError::from(node))),
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
