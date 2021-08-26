use crate::TryFromPrimitiveError;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::{convert::TryFrom, str::Chars};
use thiserror::Error;

/// A kanji classification using the Four Corner system.
/// http://www.edrdg.org/wwwjdic/FOURCORNER.html
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FourCorner {
    /// The stroke at the top left corner.
    pub top_left: Stroke,
    /// The stroke at the top right corner.
    pub top_right: Stroke,
    /// The stroke at the bottom left corner.
    pub bottom_left: Stroke,
    /// The stroke at the bottom right corner.
    pub bottom_right: Stroke,
    /// Where necessary to differentiate between other
    /// characters with the same strokes, this extra stroke
    /// is found above the bottom right stroke.
    ///
    /// In the database, we only ever see this with the fifth
    /// corner. Still, not including it is technically
    /// allowed, so I include it here for generality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fifth_corner: Option<Stroke>,
}

/// A stroke shape in the Four Corner system.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    TryFromPrimitive,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
)]
#[repr(u8)]
pub enum Stroke {
    /// 亠
    Lid,
    /// 一
    LineHorizontal,
    /// ｜
    LineVertical,
    /// 丶
    Dot,
    /// 十
    Cross,
    /// キ
    Skewer,
    /// 口
    Box,
    /// 厂
    Angle,
    /// 八
    Hachi,
    /// 小
    Chiisai,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FourCornerParseError {
    #[error("(Four corner) Failed to extract a stroke: {0}")]
    Stroke(#[from] TryFromPrimitiveError<Stroke>),
    #[error("(Four corner) Too few characters for four corners code")]
    ToFewCharacters,
    #[error("(Four corner) Expected a digit")]
    Digit,
    #[error("(Four corner) Expected a period delimiting the fifth corner")]
    Pattern,
}

impl TryFrom<&str> for FourCorner {
    type Error = FourCornerParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let mut iter = text.chars();
        let top_left = take_stroke(&mut iter)?;
        let top_right = take_stroke(&mut iter)?;
        let bottom_left = take_stroke(&mut iter)?;
        let bottom_right = take_stroke(&mut iter)?;
        if iter.next() != Some('.') {
            return Err(FourCornerParseError::Pattern);
        }
        let fifth_corner = take_stroke(&mut iter)?;
        Ok(FourCorner {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            fifth_corner: Some(fifth_corner),
        })
    }
}

fn take_stroke(chars: &mut Chars) -> Result<Stroke, FourCornerParseError> {
    let int: u8 = char_to_u8(chars.next().ok_or(FourCornerParseError::ToFewCharacters)?)?;
    let stroke = Stroke::try_from(int)?;
    Ok(stroke)
}

fn char_to_u8(c: char) -> Result<u8, FourCornerParseError> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        _ => Err(FourCornerParseError::Digit),
    }
}
