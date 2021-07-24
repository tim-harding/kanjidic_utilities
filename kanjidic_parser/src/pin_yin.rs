use crate::shared::IResult;
use nom::{bytes::complete::take_while1, sequence::tuple};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

use crate::shared::take_uint;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum PinYinError {
    #[error("Node contains no text")]
    NoText,
    #[error("Pin yin tones not recognized")]
    InvalidTone(#[from] TryFromPrimitiveError<Tone>),
    #[error("Unrecognized pin yin format")]
    Format,
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

impl<'a, 'input> TryFrom<Node<'a, 'input>> for PinYin<'a> {
    type Error = PinYinError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = node.text().ok_or(PinYinError::NoText)?;
        let (_i, (romanization, tone)) = parts(text).map_err(|_| PinYinError::Format)?;
        let tone = Tone::try_from(tone)?;
        Ok(Self { romanization, tone })
    }
}

fn parts(s: &str) -> IResult<(&str, u8)> {
    tuple((take_while1(|c: char| c.is_ascii_alphabetic()), take_uint))(s)
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
