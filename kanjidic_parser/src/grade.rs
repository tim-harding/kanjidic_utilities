use std::convert::TryFrom;
use kanjidic_types::Grade;
use roxmltree::Node;
use thiserror::Error;

use crate::{
    pos_error::PosError,
    shared::{text_uint, SharedError},
};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum GradeError {
    #[error("(Grade) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Grade) Parsing: {0}, {1}")]
    Parse(PosError, GradeStrError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum GradeStrError {
    #[error("(Grade) {0} is not a recognized grade level")]
    Unrecognized(u8),
}

impl TryFrom<u8> for Grade {
    type Error = GradeStrError;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            1..=6 => Ok(Grade::Kyouiku(n)),
            8 => Ok(Grade::Jouyou),
            9 => Ok(Grade::Jinmeiyou),
            10 => Ok(Grade::JinmeiyouJouyouVariant),
            n => Err(GradeStrError::Unrecognized(n)),
        }
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Grade {
    type Error = GradeError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let n: u8 = text_uint(node)?;
        Self::try_from(n).map_err(|err| GradeError::Parse(PosError::from(node), err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn four_corner() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("grade"))
            .unwrap();
        let grade = Grade::try_from(node);
        assert_eq!(grade, Ok(Grade::Jouyou))
    }
}
