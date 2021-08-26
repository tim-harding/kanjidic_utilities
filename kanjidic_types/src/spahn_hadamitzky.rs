use std::convert::TryFrom;

use crate::{shared::NomErrorReason, take_uint, IResult, NomErr};
use nom::{bytes::complete::take, character::complete::char, sequence::tuple};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Todo: Enum for radical

/// Descriptor code for The Kanji Dictionary.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ShParseError {
    #[error("(Spahn Hadamitzky) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for ShParseError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

// They are in the form nxnn.n,
// e.g.  3k11.2, where the  kanji has 3 strokes in the
// identifying radical, it is radical "k" in the SH
// classification system, there are 11 other strokes, and it is
// the 2nd kanji in the 3k sequence.

impl TryFrom<&str> for ShDesc {
    type Error = ShParseError;

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

fn parts(s: &str) -> IResult<(u8, &str, u8, char, u8)> {
    tuple((take_uint, take(1u8), take_uint, char('.'), take_uint))(s)
}
