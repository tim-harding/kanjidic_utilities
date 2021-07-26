use roxmltree::Node;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

use crate::{
    pos_error::PosError,
    shared::{text_uint, SharedError},
};

#[derive(Debug, PartialEq, Eq, Error)]
pub enum StrokeCountError {
    #[error("(Stroke count) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Stroke count) Expected at least one entry: {0}")]
    Accepted(PosError),
}

/// The number of strokes in a kanji.
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StrokeCount {
    /// The accepted number of strokes.
    pub accepted: u8,
    /// Possible miscounts of the stroke count.
    pub miscounts: Vec<u8>,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for StrokeCount {
    type Error = StrokeCountError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let mut children = node
            .children()
            .filter(|child| child.has_tag_name("stroke_count"))
            .map(|child| Ok(text_uint(child)?));
        let accepted = children
            .next()
            .ok_or_else(|| StrokeCountError::Accepted(PosError::from(node)))??;
        let miscounts: Result<Vec<u8>, StrokeCountError> = children.collect();
        let miscounts = miscounts?;
        Ok(Self {
            accepted,
            miscounts,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{stroke_count::StrokeCount, test_shared::DOC};
    use std::convert::TryFrom;

    #[test]
    fn stroke_count() {
        let character = DOC
            .descendants()
            .find(|node| node.has_tag_name("misc"))
            .unwrap();
        let count = StrokeCount::try_from(character);
        assert_eq!(
            count,
            Ok(StrokeCount {
                accepted: 7,
                miscounts: vec![],
            })
        )
    }
}
