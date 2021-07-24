use nom::{character::complete::char, sequence::tuple};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

use crate::{pos_error::PosError, shared::{self, IResult, SharedError, take_uint}};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SkipError {
    #[error("Invalid solid pattern: {0}")]
    InvalidSolidPattern(#[from] TryFromPrimitiveError<SolidSubpattern>),
    #[error("Shared utility error: {0}")]
    Shared(#[from] SharedError),
    #[error("Did not fit the format for a skip code")]
    Format(PosError),
    #[error("The digit indicating the pattern was not valid")]
    SkipKind(PosError),
}

/// Kanji code from the SKIP system of indexing.
/// http://www.edrdg.org/wwwjdic/SKIP.html
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Skip {
    type Error = SkipError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let text = shared::text(node)?;
        let (_i, (pattern_kind, _, first, _, second)) =
            parts(text).map_err(|_| SkipError::Format(PosError::from(node)))?;
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
                let solid_subpattern =
                    SolidSubpattern::try_from(second)?;
                Ok(Skip::Solid(SkipSolid {
                    total_stroke_count: first,
                    solid_subpattern,
                }))
            }
            _ => Err(SkipError::SkipKind(PosError::from(node))),
        }
    }
}

fn parts(s: &str) -> IResult<(u8, char, u8, char, u8)> {
    tuple((take_uint, char('-'), take_uint, char('-'), take_uint))(s)
}

/// Left and right parts of the kanji.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkipHorizontal {
    /// Number of strokes in the left part.
    pub left: u8,
    /// Number of strokes in the right part.
    pub right: u8,
}

/// Top and bottom parts of the kanji.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkipVertical {
    /// Number of strokes in the top part.
    pub top: u8,
    /// Number of strokes in the bottom part.
    pub bottom: u8,
}

/// Interior and exterior parts of the kanji.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkipEnclosure {
    /// Number of strokes in the exterior part.
    pub exterior: u8,
    /// Number of strokes in the interior part.
    pub interior: u8,
}

/// Classification for kanji that don't fit another pattern.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkipSolid {
    /// The total number of strokes in the kanji.
    pub total_stroke_count: u8,
    /// The subpattern that defines the kanji.
    pub solid_subpattern: SolidSubpattern,
}

/// An identifying characteristic of the kanji.
#[derive(TryFromPrimitive, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
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
