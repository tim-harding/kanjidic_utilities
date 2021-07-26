use crate::{
    pos_error::PosError,
    shared::{self, IResult, NomErr, NomErrorReason, SharedError},
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::streaming::one_of,
    combinator::{map, recognize, value},
    multi::many_till,
};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

use crate::shared::take_uint;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum PinYinError {
    #[error("(Pin Yin) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Pin Yin) Parsing: {0}, {1}")]
    Parse(PosError, PinYinStrError),
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum PinYinStrError {
    #[error("(Pin yin) Tone not recognized: {0}")]
    InvalidTone(#[from] TryFromPrimitiveError<Tone>),
    #[error("(Pin yin) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for PinYinStrError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

// A modern PinYin romanization of the Chinese reading.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
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
    Serialize,
    Deserialize,
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

impl TryFrom<&str> for PinYin {
    type Error = PinYinStrError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, (romanization, tone)) = parts(text)?;
        let tone = Tone::try_from(tone)?;
        Ok(Self { romanization, tone })
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for PinYin {
    type Error = PinYinError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = shared::text(node)?;
        Self::try_from(text).map_err(|err| PinYinError::Parse(PosError::from(node), err))
    }
}

fn parts(s: &str) -> IResult<(String, u8)> {
    map(pronunciation_parts, |(parts, tone)| (parts.join(""), tone))(s)
}

fn pronunciation_parts(s: &str) -> IResult<(Vec<&str>, u8)> {
    many_till(alt((umlaut, carrot, special_letter, letters)), take_uint)(s)
}

// Todo: Check that this is working correctly.
// Tests do not currently cover this.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn pin_yin() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("reading")
                    && node
                        .attribute("r_type")
                        .map(|value| value.eq("pinyin"))
                        .unwrap_or(false)
            })
            .unwrap();
        let pin_yin = PinYin::try_from(node);
        assert_eq!(
            pin_yin,
            Ok(PinYin {
                romanization: "ya".into(),
                tone: Tone::Falling,
            })
        )
    }
}
