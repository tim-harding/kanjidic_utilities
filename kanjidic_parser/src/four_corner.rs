use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};
use kanjidic_types::{FourCorner, Stroke, TryFromPrimitiveError};
use roxmltree::Node;
use std::{convert::TryFrom, str::Chars};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FourCornerError {
    #[error("(Four corner) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Four corner) Parsing: {0}, {1}")]
    Str(PosError, FourCornerStrError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FourCornerStrError {
    #[error("(Four corner) Failed to extract a stroke: {0}")]
    Stroke(#[from] TryFromPrimitiveError<Stroke>),
    #[error("(Four corner) Too few characters for four corners code")]
    ToFewCharacters,
    #[error("(Four corner) Expected a digit")]
    Digit,
    #[error("(Four corner) Expected a period delimiting the fifth corner")]
    Pattern,
}

fn from_str(text: &str) -> Result<FourCorner, FourCornerStrError> {
    let mut iter = text.chars();
    let top_left = take_stroke(&mut iter)?;
    let top_right = take_stroke(&mut iter)?;
    let bottom_left = take_stroke(&mut iter)?;
    let bottom_right = take_stroke(&mut iter)?;
    if iter.next() != Some('.') {
        return Err(FourCornerStrError::Pattern);
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

pub fn from(node: Node) -> Result<FourCorner, FourCornerError> {
    let text = shared::text(node)?;
    from_str(text).map_err(|err| FourCornerError::Str(PosError::from(node), err))
}

fn take_stroke(chars: &mut Chars) -> Result<Stroke, FourCornerStrError> {
    let int: u8 = char_to_u8(chars.next().ok_or(FourCornerStrError::ToFewCharacters)?)?;
    let stroke = Stroke::try_from(int)?;
    Ok(stroke)
}

fn char_to_u8(c: char) -> Result<u8, FourCornerStrError> {
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
        _ => Err(FourCornerStrError::Digit),
    }
}

#[cfg(test)]
mod tests {
    use kanjidic_types::{FourCorner, Stroke};

    use super::from;
    use crate::test_shared::DOC;

    #[test]
    fn four_corner() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("q_code")
                    && node
                        .attribute("qc_type")
                        .map(|value| value.eq("four_corner"))
                        .unwrap_or(false)
            })
            .unwrap();
        let four_corner = from(node);
        assert_eq!(
            four_corner,
            Ok(FourCorner {
                top_left: Stroke::LineHorizontal,
                top_right: Stroke::Lid,
                bottom_left: Stroke::LineHorizontal,
                bottom_right: Stroke::Lid,
                fifth_corner: Some(Stroke::Box),
            })
        )
    }
}
