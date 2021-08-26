use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::convert::TryFrom;
use thiserror::Error;

/// Identification of a kanji in the De Roo system.
/// http://www.edrdg.org/wwwjdic/deroo.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DeRoo {
    /// The graphic element that appears at the top of the kanji.
    pub top: ExtremeTop,
    /// The graphic element that appears at the bottom of the kanji.
    pub bottom: ExtremeBottom,
}

#[derive(
    TryFromPrimitive,
    Eq,
    PartialEq,
    Debug,
    Clone,
    Copy,
    PartialOrd,
    Ord,
    Hash,
    Serialize_repr,
    Deserialize_repr,
)]
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

#[derive(
    TryFromPrimitive,
    Eq,
    PartialEq,
    Debug,
    Clone,
    Copy,
    PartialOrd,
    Ord,
    Hash,
    Serialize_repr,
    Deserialize_repr,
)]
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
    BottomCorner,
    Mountain,
    Mouth,
    Sun,
    Eye,
}

#[derive(Debug, Error, Eq, PartialEq, Clone)]
pub enum DeRooParseError {
    #[error("(De Roo) Should be three or four digits, not {0}")]
    InvalidLength(usize),
    #[error("(De Roo) Could not parse part of the code as a number")]
    Number,
    #[error("(De Roo) Subslice could not be treated as UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("(De Roo) Extreme top: {0}")]
    ExtremeTop(#[from] TryFromPrimitiveError<ExtremeTop>),
    #[error("(De Roo) Extreme bottom: {0}")]
    ExtremeBottom(#[from] TryFromPrimitiveError<ExtremeBottom>),
}

impl TryFrom<&str> for DeRoo {
    type Error = DeRooParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        match text.len() {
            3 => from_slices(text, 1),
            4 => from_slices(text, 2),
            n => Err(DeRooParseError::InvalidLength(n)),
        }
    }
}

fn from_slices(text: &str, first: usize) -> Result<DeRoo, DeRooParseError> {
    let top = ExtremeTop::try_from(u8_from_slice(text, 0, first)?)?;
    let bottom = ExtremeBottom::try_from(u8_from_slice(text, first, 2)?)?;
    Ok(DeRoo { top, bottom })
}

fn u8_from_slice(text: &str, start: usize, count: usize) -> Result<u8, DeRooParseError> {
    let top = &text.as_bytes()[start..start + count];
    let top = std::str::from_utf8(top)?;
    let top: u8 = top.parse().map_err(|_| DeRooParseError::Number)?;
    Ok(top)
}
