use kanjidic_types::{DeRoo, ExtremeBottom, ExtremeTop, TryFromPrimitiveError};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

use crate::{
    pos_error::PosError,
    shared::{text, SharedError},
};

#[derive(Debug, Error, Eq, PartialEq, Clone)]
pub enum DeRooError {
    #[error("(De Roo) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(De Roo) Parsing: {0}, {1}")]
    Str(PosError, DeRooStrError),
}

#[derive(Debug, Error, Eq, PartialEq, Clone)]
pub enum DeRooStrError {
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

pub fn from(node: Node) -> Result<DeRoo, DeRooError> {
    from_str(text(node)?).map_err(|err| DeRooError::Str(PosError::from(node), err))
}

fn from_str(text: &str) -> Result<DeRoo, DeRooStrError> {
    match text.len() {
        3 => from_slices(text, 1),
        4 => from_slices(text, 2),
        n => Err(DeRooStrError::InvalidLength(n)),
    }
}

fn from_slices(text: &str, first: usize) -> Result<DeRoo, DeRooStrError> {
    let top = ExtremeTop::try_from(u8_from_slice(text, 0, first)?)?;
    let bottom = ExtremeBottom::try_from(u8_from_slice(text, first, 2)?)?;
    Ok(DeRoo { top, bottom })
}

fn u8_from_slice(text: &str, start: usize, count: usize) -> Result<u8, DeRooStrError> {
    let top = &text.as_bytes()[start..start + count];
    let top = std::str::from_utf8(top)?;
    let top: u8 = top.parse().map_err(|_| DeRooStrError::Number)?;
    Ok(top)
}

#[cfg(test)]
mod tests {
    use kanjidic_types::{DeRoo, ExtremeBottom, ExtremeTop};

    use super::from;
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
        let deroo = from(node);
        assert_eq!(
            deroo,
            Ok(DeRoo {
                top: ExtremeTop::Bald,
                bottom: ExtremeBottom::StandingBottom,
            })
        )
    }
}
