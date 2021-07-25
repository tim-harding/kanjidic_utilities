use crate::{
    pos_error::PosError,
    shared::{self, IResult, NomErr, NomErrorReason, SharedError},
};
use nom::{branch::alt, bytes::complete::{tag, take_while1}, combinator::value, sequence::tuple};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use roxmltree::Node;
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct PinYin<'a> {
    /// The romanized reading.
    pub romanization: &'a str,
    /// The Mandarin tone of the reading.
    pub tone: Tone,
}

/// One of the four tones of Mandarin.
/// https://en.wikipedia.org/wiki/Standard_Chinese_phonology#Tones
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, TryFromPrimitive)]
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

impl<'a, 'b: 'a> TryFrom<&'b str> for PinYin<'a> {
    type Error = PinYinStrError;

    fn try_from(text: &'b str) -> Result<Self, Self::Error> {
        let (_i, (romanization, tone)) = parts(text)?;
        let tone = Tone::try_from(tone)?;
        Ok(Self { romanization, tone })
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for PinYin<'a> {
    type Error = PinYinError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = shared::text(node)?;
        Self::try_from(text).map_err(|err| PinYinError::Parse(PosError::from(node), err))
    }
}

fn parts(s: &str) -> IResult<(&str, u8)> {
    tuple((pronunciation, take_uint))(s)
}

fn pronunciation(s: &str) -> IResult<&str> {
    alt((umlauted, plain_roman))(s)
}

// If the romanization takes the form 'lu:' or 'nu:',
// I am pretty sure that is meant to be interpreted as
// the pronunciations lü and nü
fn umlauted(s: &str) -> IResult<&str> {
    alt((value("lü", tag("lu:")), value("nü", tag("nu:"))))(s)
}

fn plain_roman(s: &str) -> IResult<&str> {
    take_while1(|c: char| c.is_ascii_alphabetic())(s)
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
                romanization: "ya",
                tone: Tone::Falling,
            })
        )
    }
}
