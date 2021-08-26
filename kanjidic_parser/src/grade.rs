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
    Parse(PosError, GradeU8Error),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum GradeU8Error {
    #[error("(Grade) {0} is not a recognized grade level")]
    Unrecognized(u8),
}

pub fn from(node: Node) -> Result<Grade, GradeError> {
    let n: u8 = text_uint(node)?;
    from_u8(n).map_err(|err| GradeError::Parse(PosError::from(node), err))
}

fn from_u8(n: u8) -> Result<Grade, GradeU8Error> {
    match n {
        1..=6 => Ok(Grade::Kyouiku(n)),
        8 => Ok(Grade::Jouyou),
        9 => Ok(Grade::Jinmeiyou),
        10 => Ok(Grade::JinmeiyouJouyouVariant),
        n => Err(GradeU8Error::Unrecognized(n)),
    }
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::Grade;

    #[test]
    fn four_corner() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("grade"))
            .unwrap();
        let grade = from(node);
        assert_eq!(grade, Ok(Grade::Jouyou))
    }
}
