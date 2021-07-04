use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum DeRooError {
    #[error("The node contained no text")]
    NoText,
    #[error("A De Roo code should be three or four digits")]
    InvalidLength,
    #[error("Could not parse part of the code as a number")]
    Number,
    #[error("Subslice could not be treated as UTF-8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("The extreme top code was not valid")]
    ExtremeTop(#[from] TryFromPrimitiveError<ExtremeTop>),
    #[error("The extreme bottom code was not valid")]
    ExtremeBottom(#[from] TryFromPrimitiveError<ExtremeBottom>),
}

/// Identification of a kanji in the De Roo system.
/// http://www.edrdg.org/wwwjdic/deroo.html
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DeRoo {
    /// The graphic element that appears at the top of the kanji.
    pub top: ExtremeTop,

    /// The graphic element that appears at the bottom of the kanji.
    pub bottom: ExtremeBottom,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for DeRoo {
    type Error = DeRooError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = node.text().ok_or(DeRooError::NoText)?;
        match text.len() {
            3 => from_slices(text, 1),
            4 => from_slices(text, 2),
            _ => Err(DeRooError::InvalidLength),
        }
    }
}

fn from_slices(text: &str, first: usize) -> Result<DeRoo, DeRooError> {
    let top = ExtremeTop::try_from(u8_from_slice(text, 0, first)?)?;
    let bottom = ExtremeBottom::try_from(u8_from_slice(text, first, 2)?)?;
    Ok(DeRoo { top, bottom })
}

fn u8_from_slice(text: &str, start: usize, count: usize) -> Result<u8, DeRooError> {
    let top = &text.as_bytes()[start..start + count];
    let top = std::str::from_utf8(top)?;
    let top: u8 = top.parse().map_err(|_| DeRooError::Number)?;
    Ok(top)
}

#[derive(TryFromPrimitive, Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ExtremeTop {
    // Dot
    Dot = 3,
    RoofDot,
    DottedCliff,
    Altar,
    KanaU,
    Lid,
    Horns,

    // Vertical line
    SmallOnBox,
    Small,
    VerticalLine,
    HandOnTheLeft,
    Cross,
    CrossOnBox,
    KanaKa,
    Woman,
    Tree,
    LetterH,

    // Diagonal line
    KanaNo,
    ManOnTheLeft,
    Thousand,
    ManOnTheTop,
    Cow,
    KanaKu,
    HillTop,
    LeftArrow,
    RoofDiagonalLine,
    X,

    // Horizontal line
    HorizontalLine,
    Fourth,
    Bald,
    Cliff,
    TopLeftCorner,
    TopRightCorner,
    UpsideDownCan,
    Mouth,
    Sun,
    EyeTop,
}

#[derive(TryFromPrimitive, Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum ExtremeBottom {
    // Dot
    FourDots = 40,
    Small,
    Water,

    // Left hook
    KanaRi,
    Seal,
    SwordBottom,
    Moon,
    DotlessInch,
    Inch,
    MouthLeftHook,
    BirdBottom,
    Animal,
    BowBottom,
    LeftHook,

    // Vertical line
    VerticalLine,
    Cross,

    // Right hook
    RightHook,
    Legs,
    Heart,
    TasseledSpearBottom,

    // Diagonal line
    KanaNo,

    // Back diagonal line
    SmallPodium,
    BackKanaNo,
    Big,
    Tree,
    SmallSpoon,
    Govern,
    Again,
    WindyAgain,
    Woman,

    // Head bottom
    HeadBottom,

    // Watakushi bottom
    WatakushiBottom,

    // Horizontal line
    HorizontalLine,
    StandingBottom,
    DishBottom,
    Mountain,
    Mouth,
    Sun,
    Eye,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn de_roo() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("q_code")
                    && node
                        .attribute("qc_type")
                        .map(|value| value.eq("deroo"))
                        .unwrap_or(false)
            })
            .unwrap();
        let deroo = DeRoo::try_from(node);
        assert_eq!(
            deroo,
            Ok(DeRoo {
                top: ExtremeTop::Bald,
                bottom: ExtremeBottom::StandingBottom,
            })
        )
    }
}
