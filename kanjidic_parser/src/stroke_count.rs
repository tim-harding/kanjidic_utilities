use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum StrokeCountError {
    #[error("Node contains no text")]
    NoText,
    #[error("Could not parse node text as a number")]
    Number,
    #[error("Expected at least one entry for stroke count")]
    Accepted,
}

/// The number of strokes in a kanji.
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
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
            .descendants()
            .filter(|child| child.has_tag_name("stroke_count"))
            .map(|child| {
                let text = child.text().ok_or(StrokeCountError::NoText)?;
                let strokes: u8 = text.parse().map_err(|_| StrokeCountError::Number)?;
                Ok(strokes)
            });
        let accepted = children.next().ok_or(StrokeCountError::Accepted)??;
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
    fn parses_stroke_count() {
        let character = DOC
            .descendants()
            .find(|node| node.has_tag_name("character"))
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
