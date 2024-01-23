use super::radical::Radical;
use crate::{shared::NomErrorReason, take_uint, IResult, NomErr};
use nom::{bytes::complete::take, character::complete::char, combinator::map_res, sequence::tuple};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

/// Descriptor code for The Kanji Dictionary.
/// The code reference can be found here: <http://www.edrdg.org/wiki/index.php/KANJIDIC_Project>
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Descriptor {
    /// Number of strokes in the identifying radical.
    pub radical_strokes: u8,
    /// The letter for the radical in the identification system.
    pub radical: Radical,
    /// The number of strokes not included in the radical.
    pub other_strokes: u8,
    /// The position of the kanji in the sequence described
    /// by the other descriptor parts.
    pub sequence: u8,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ParseError {
    #[error("(Spahn Hadamitzky) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for ParseError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

impl TryFrom<&str> for Descriptor {
    type Error = ParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, (radical_strokes, radical, other_strokes, _, sequence)) = parts(text)?;
        Ok(Self {
            radical_strokes,
            radical,
            other_strokes,
            sequence,
        })
    }
}

fn parts(s: &str) -> IResult<(u8, Radical, u8, char, u8)> {
    tuple((take_uint, radical, take_uint, char('.'), take_uint))(s)
}

fn radical(s: &str) -> IResult<Radical> {
    map_res(take(1u8), |s: &str| {
        let c = s.chars().next().unwrap();
        Radical::try_from(c)
    })(s)
}
