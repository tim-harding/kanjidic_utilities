use crate::shared::{text_uint, SharedError};
use kanjidic_types::Grade;
use roxmltree::Node;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("(Grade) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Grade) {0} is not a recognized grade level")]
    Unrecognized(u8),
}

pub fn from(node: Node) -> Result<Grade, Error> {
    let n: u8 = text_uint(&node)?;
    match n {
        1..=6 => Ok(Grade::Kyouiku(n)),
        8 => Ok(Grade::Jouyou),
        9 => Ok(Grade::Jinmeiyou),
        10 => Ok(Grade::JinmeiyouJouyouVariant),
        n => Err(Error::Unrecognized(n)),
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
