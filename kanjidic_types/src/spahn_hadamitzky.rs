use std::convert::TryFrom;

use crate::{shared::NomErrorReason, take_uint, IResult, NomErr};
use nom::{bytes::complete::take, character::complete::char, combinator::map_res, sequence::tuple};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Todo: Enum for radical

/// Descriptor code for The Kanji Dictionary.
/// The code reference can be found here: <http://www.edrdg.org/wiki/index.php/KANJIDIC_Project>
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShDesc {
    /// Number of strokes in the identifying radical.
    pub radical_strokes: u8,
    /// The letter for the radical in the identification system.
    pub radical: ShRadical,
    /// The number of strokes not included in the radical.
    pub other_strokes: u8,
    /// The position of the kanji in the sequence described
    /// by the other descriptor parts.
    pub sequence: u8,
}

/// An identifying radical in the Spahn and Hadamitzky classification system.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShRadical {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ShRadicalParseError {
    #[error("(ShRadical) Unknown radical")]
    UnknownChar,
}

impl TryFrom<char> for ShRadical {
    type Error = ShRadicalParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(ShRadical::A),
            'b' => Ok(ShRadical::B),
            'c' => Ok(ShRadical::C),
            'd' => Ok(ShRadical::D),
            'e' => Ok(ShRadical::E),
            'f' => Ok(ShRadical::F),
            'g' => Ok(ShRadical::G),
            'h' => Ok(ShRadical::H),
            'i' => Ok(ShRadical::I),
            'j' => Ok(ShRadical::J),
            'k' => Ok(ShRadical::K),
            'l' => Ok(ShRadical::L),
            'm' => Ok(ShRadical::M),
            'n' => Ok(ShRadical::N),
            'o' => Ok(ShRadical::O),
            'p' => Ok(ShRadical::P),
            'q' => Ok(ShRadical::Q),
            'r' => Ok(ShRadical::R),
            's' => Ok(ShRadical::S),
            't' => Ok(ShRadical::T),
            'u' => Ok(ShRadical::U),
            'v' => Ok(ShRadical::V),
            'w' => Ok(ShRadical::W),
            'x' => Ok(ShRadical::X),
            'y' => Ok(ShRadical::Y),
            'z' => Ok(ShRadical::Z),
            _ => Err(ShRadicalParseError::UnknownChar),
        }
    }
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

impl TryFrom<&str> for ShDesc {
    type Error = ShParseError;

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

fn parts(s: &str) -> IResult<(u8, ShRadical, u8, char, u8)> {
    tuple((take_uint, radical, take_uint, char('.'), take_uint))(s)
}

fn radical(s: &str) -> IResult<ShRadical> {
    map_res(take(1u8), |s: &str| {
        let c = s.chars().next().unwrap();
        ShRadical::try_from(c)
    })(s)
}
