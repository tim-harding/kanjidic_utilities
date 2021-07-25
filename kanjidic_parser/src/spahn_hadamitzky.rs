use crate::{
    pos_error::PosError,
    shared::{self, take_uint, IResult, NomErr, NomErrorReason, SharedError},
};
use nom::{bytes::complete::take, character::complete::char, sequence::tuple};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ShError {
    #[error("Error from shared utilities: {0}")]
    Shared(#[from] SharedError),
    #[error("Error while parsing Spahn Hadamitzky descriptor: {0}")]
    Str(PosError, ShStrError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ShStrError {
    #[error("Invalid Spahn Hadamitzky descriptor")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for ShStrError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

// They are in the form nxnn.n,
// e.g.  3k11.2, where the  kanji has 3 strokes in the
// identifying radical, it is radical "k" in the SH
// classification system, there are 11 other strokes, and it is
// the 2nd kanji in the 3k11 sequence.

/// Descriptor code for The Kanji Dictionary.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
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

impl TryFrom<&str> for ShDesc {
    type Error = ShStrError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, (radical_strokes, radical, other_strokes, _, sequence)) = parts(text)?;
        let radical = radical.chars().next().unwrap();
        Ok(Self {
            radical_strokes,
            radical,
            other_strokes,
            sequence,
        })
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for ShDesc {
    type Error = ShError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = shared::text(node)?;
        Self::try_from(text).map_err(|err| ShError::Str(PosError::from(node), err))
    }
}

fn parts(s: &str) -> IResult<(u8, &str, u8, char, u8)> {
    tuple((take_uint, take(1u8), take_uint, char('.'), take_uint))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn spahn_hadamitzky() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("variant")
                    && node
                        .attribute("var_type")
                        .map(|value| value.eq("s_h"))
                        .unwrap_or(false)
            })
            .unwrap();
        let sh = ShDesc::try_from(node);
        assert_eq!(
            sh,
            Ok(ShDesc {
                radical_strokes: 2,
                radical: 'k',
                other_strokes: 4,
                sequence: 6,
            })
        )
    }
}
