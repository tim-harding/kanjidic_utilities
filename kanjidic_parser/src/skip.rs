use crate::{
    pos_error::PosError,
    shared::{self, take_uint, IResult, NomErr, NomErrorReason, SharedError},
};
use kanjidic_types::{
    Skip, SkipEnclosure, SkipHorizontal, SkipSolid, SkipVertical, SolidSubpattern,
    TryFromPrimitiveError,
};
use nom::{character::complete::char, sequence::tuple};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

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

fn from_str(text: &str) -> Result<Skip, SkipStrError> {
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

pub fn from(node: Node) -> Result<Skip, SkipError> {
    let text = shared::text(node)?;
    from_str(text).map_err(|err| SkipError::Parse(PosError::from(node), err))
}

fn parts(s: &str) -> IResult<(u8, char, u8, char, u8)> {
    tuple((take_uint, char('-'), take_uint, char('-'), take_uint))(s)
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{Skip, SkipSolid, SolidSubpattern};

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
        let skip = from(node);
        assert_eq!(
            skip,
            Ok(Skip::Solid(SkipSolid {
                total_stroke_count: 7,
                solid_subpattern: SolidSubpattern::TopLine,
            }))
        )
    }
}
