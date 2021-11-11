use crate::{
    de_roo, four_corner,
    pos_error::PosError,
    shared::{attr, SharedError},
    skip, spahn_hadamitzky, DeRooError, FourCornerError, ShError, SkipError,
};
use kanjidic_types::{Misclassification, MisclassificationKind, QueryCode};
use roxmltree::Node;
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

pub fn from(node: Node) -> Result<QueryCode, QueryCodeError> {
    let qc_type = attr(&node, "qc_type")?;
    match qc_type {
        "skip" => {
            if let Some(misclass_kind) = node.attribute("skip_misclass") {
                Ok(QueryCode::Misclassification(match misclass_kind {
                    "posn" => Ok(Misclassification {
                        kind: MisclassificationKind::Position,
                        skip: skip::from(node)?,
                    }),
                    "stroke_count" => Ok(Misclassification {
                        kind: MisclassificationKind::StrokeCount,
                        skip: skip::from(node)?,
                    }),
                    "stroke_and_posn" => Ok(Misclassification {
                        kind: MisclassificationKind::StrokeAndPosition,
                        skip: skip::from(node)?,
                    }),
                    "stroke_diff" => Ok(Misclassification {
                        kind: MisclassificationKind::Ambiguous,
                        skip: skip::from(node)?,
                    }),
                    _ => Err(QueryCodeError::UnknownMisclassification(PosError::from(
                        &node,
                    ))),
                }?))
            } else {
                Ok(QueryCode::Skip(skip::from(node)?))
            }
        }
        "sh_desc" => Ok(QueryCode::SpahnHadamitzky(spahn_hadamitzky::from(node)?)),
        "four_corner" => Ok(QueryCode::FourCorner(four_corner::from(node)?)),
        "deroo" => Ok(QueryCode::DeRoo(de_roo::from(node)?)),
        _ => Err(QueryCodeError::UnknownType(PosError::from(&node))),
    }
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{QueryCode, Skip, SkipSolid, SolidSubpattern};

    #[test]
    fn query_code() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("q_code"))
            .unwrap();
        let query_code = from(node);
        assert_eq!(
            query_code,
            Ok(QueryCode::Skip(Skip::Solid(SkipSolid {
                total_stroke_count: 7,
                solid_subpattern: SolidSubpattern::TopLine,
            })))
        )
    }
}
