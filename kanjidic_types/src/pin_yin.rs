use crate::{shared::NomErrorReason, take_uint, IResult, NomErr, TryFromPrimitiveError};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::streaming::one_of,
    combinator::{map, recognize, value},
    multi::many_till,
};
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::convert::TryFrom;
use thiserror::Error;

// A modern PinYin romanization of the Chinese reading.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinYin {
    /// The romanized reading.
    pub romanization: String,
    /// The Mandarin tone of the reading.
    pub tone: Tone,
}

/// One of the four tones of Mandarin.
/// https://en.wikipedia.org/wiki/Standard_Chinese_phonology#Tones
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    TryFromPrimitive,
    Serialize_repr,
    Deserialize_repr,
)]
#[repr(u8)]
pub enum Tone {
    /// A steady high sound
    High = 1,
    /// A rising tone
    Rising,
    /// A low or dipping tone
    Low,
    /// A sharp falling tone
    Falling,
    /// A lack of tone
    Neutral,
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum PinYinParseError {
    #[error("(Pin yin) Tone not recognized: {0}")]
    InvalidTone(#[from] TryFromPrimitiveError<Tone>),
    #[error("(Pin yin) Format: {0}")]
    Format(NomErrorReason),
}

// Todo: move this one back to parsing and check for others

impl<'a> From<NomErr<'a>> for PinYinParseError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

impl TryFrom<&str> for PinYin {
    type Error = PinYinParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, (romanization, tone)) = parts(text)?;
        let tone = Tone::try_from(tone)?;
        Ok(PinYin { romanization, tone })
    }
}

fn parts(s: &str) -> IResult<(String, u8)> {
    map(pronunciation_parts, |(parts, tone)| (parts.join(""), tone))(s)
}

fn pronunciation_parts(s: &str) -> IResult<(Vec<&str>, u8)> {
    many_till(alt((umlaut, carrot, special_letter, letters)), take_uint)(s)
}

fn umlaut(s: &str) -> IResult<&str> {
    value("ü", tag("u:"))(s)
}

fn carrot(s: &str) -> IResult<&str> {
    value("ê", tag("e^"))(s)
}

fn special_letter(s: &str) -> IResult<&str> {
    recognize(one_of("ue"))(s)
}

fn letters(s: &str) -> IResult<&str> {
    take_while1(|c: char| c != 'u' && c != 'e' && c.is_ascii_alphabetic())(s)
}
