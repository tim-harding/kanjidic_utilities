use crate::{
    pos_error::PosError,
    shared::{text_uint, SharedError},
};
use kanjidic_types::StrokeCount;
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum StrokeCountError {
    #[error("(Stroke count) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Stroke count) Expected at least one entry: {0}")]
    Accepted(PosError),
}

pub fn from(node: Node) -> Result<StrokeCount, StrokeCountError> {
    let mut children = node
        .children()
        .filter(|child| child.has_tag_name("stroke_count"))
        .map(|child| Ok(text_uint(child)?));
    let accepted = children
        .next()
        .ok_or_else(|| StrokeCountError::Accepted(PosError::from(node)))??;
    let miscounts: Result<Vec<u8>, StrokeCountError> = children.collect();
    let miscounts = miscounts?;
    Ok(StrokeCount {
        accepted,
        miscounts,
    })
}

#[cfg(test)]
mod tests {
    use kanjidic_types::StrokeCount;

    use super::from;
    use crate::test_shared::DOC;

    #[test]
    fn stroke_count() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("misc"))
            .unwrap();
        let count = from(node);
        assert_eq!(
            count,
            Ok(StrokeCount {
                accepted: 7,
                miscounts: vec![],
            })
        )
    }
}
