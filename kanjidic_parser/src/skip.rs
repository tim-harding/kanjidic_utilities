use nom::{character::complete::char, sequence::tuple};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

use crate::{
    pos_error::PosError,
    shared::{self, take_uint, IResult, NomErr, NomErrorReason, SharedError},
};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SkipError {
    #[error("(Skip) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Skip) Parsing: {0}, {1}")]
    Parse(PosError, SkipStrError),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SkipStrError {
    #[error("(Skip) Invalid solid pattern: {0}")]
    InvalidSolidPattern(#[from] TryFromPrimitiveError<SolidSubpattern>),
    #[error("(Skip) Format: {0}")]
    Format(NomErrorReason),
    #[error("(Skip) Digit indicating the pattern was not valid")]
    SkipKind,
}

impl<'a> From<NomErr<'a>> for SkipStrError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

/// Kanji code from the SKIP system of indexing.
/// http://www.edrdg.org/wwwjdic/SKIP.html
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(tag = "tag", content = "content")]
pub enum Skip {
    /// Pattern 1, the kanji can be divided into left and right parts.
    Horizontal(SkipHorizontal),
    /// Pattern 2, the kanji can be divided into top and bottom parts.
    Vertical(SkipVertical),
    /// Pattern 3, the kanji can be divided by an enclosure element.
    Enclosure(SkipEnclosure),
    /// Pattern 4, the cannot be classified by any of the above patterns.
    Solid(SkipSolid),
}

/// Left and right parts of the kanji.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SkipHorizontal {
    /// Number of strokes in the left part.
    pub left: u8,
    /// Number of strokes in the right part.
    pub right: u8,
}

/// Top and bottom parts of the kanji.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SkipVertical {
    /// Number of strokes in the top part.
    pub top: u8,
    /// Number of strokes in the bottom part.
    pub bottom: u8,
}

/// Interior and exterior parts of the kanji.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SkipEnclosure {
    /// Number of strokes in the exterior part.
    pub exterior: u8,
    /// Number of strokes in the interior part.
    pub interior: u8,
}

/// Classification for kanji that don't fit another pattern.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SkipSolid {
    /// The total number of strokes in the kanji.
    pub total_stroke_count: u8,
    /// The subpattern that defines the kanji.
    pub solid_subpattern: SolidSubpattern,
}

/// An identifying characteristic of the kanji.
#[derive(
    TryFromPrimitive,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
#[serde(tag = "tag", content = "content")]
pub enum SolidSubpattern {
    /// Contains a top line.
    TopLine = 1,
    /// Contains a bottom line.
    BottomLine,
    /// Contains a through line.
    ThroughLine,
    /// Does not contain any of the above.
    Other,
}

impl TryFrom<&str> for Skip {
    type Error = SkipStrError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, (pattern_kind, _, first, _, second)) = parts(text)?;
        match pattern_kind {
            1 => Ok(Skip::Horizontal(SkipHorizontal {
                left: first,
                right: second,
            })),
            2 => Ok(Skip::Vertical(SkipVertical {
                top: first,
                bottom: second,
            })),
            3 => Ok(Skip::Enclosure(SkipEnclosure {
                exterior: first,
                interior: second,
            })),
            4 => {
                let solid_subpattern = SolidSubpattern::try_from(second)?;
                Ok(Skip::Solid(SkipSolid {
                    total_stroke_count: first,
                    solid_subpattern,
                }))
            }
            _ => Err(SkipStrError::SkipKind),
        }
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Skip {
    type Error = SkipError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let text = shared::text(node)?;
        Self::try_from(text).map_err(|err| SkipError::Parse(PosError::from(node), err))
    }
}

fn parts(s: &str) -> IResult<(u8, char, u8, char, u8)> {
    tuple((take_uint, char('-'), take_uint, char('-'), take_uint))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn skip() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("q_code")
                    && node
                        .attribute("qc_type")
                        .map(|value| value.eq("skip"))
                        .unwrap_or(false)
            })
            .unwrap();
        let skip = Skip::try_from(node);
        assert_eq!(
            skip,
            Ok(Skip::Solid(SkipSolid {
                total_stroke_count: 7,
                solid_subpattern: SolidSubpattern::TopLine,
            }))
        )
    }
}
