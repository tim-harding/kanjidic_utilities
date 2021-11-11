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
    #[error("(Stroke count) Not all fields specified")]
    Incomplete,
}

#[derive(Default)]
pub struct StrokeCountBuilder {
    accepted: Option<u8>,
    miscounts: Vec<u8>,
}

impl StrokeCountBuilder {
    pub fn add_from_node(&mut self, node: &Node) -> Result<(), StrokeCountError> {
        let count = text_uint(node)?;
        match self.accepted {
            Some(_) => {
                self.miscounts.push(count);
            }
            None => {
                self.accepted = Some(count);
            }
        }
        Ok(())
    }

    pub fn build(self) -> Result<StrokeCount, StrokeCountError> {
        let accepted = self.accepted.ok_or(StrokeCountError::Incomplete)?;
        Ok(StrokeCount {
            accepted,
            miscounts: self.miscounts,
        })
    }
}

#[cfg(test)]
mod tests {
    use kanjidic_types::StrokeCount;
    use roxmltree::Node;

    // TODO: Refactor to use StrokeCountBuilder
    pub fn from(node: &Node) -> Result<StrokeCount, StrokeCountError> {
        let mut children = node
            .children()
            .filter(|child| child.has_tag_name("stroke_count"))
            .map(|child| Ok(text_uint(&child)?));
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

    use crate::{shared::text_uint, test_shared::DOC, PosError, StrokeCountError};

    #[test]
    fn stroke_count() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("misc"))
            .unwrap();
        let count = from(&node);
        assert_eq!(
            count,
            Ok(StrokeCount {
                accepted: 7,
                miscounts: vec![],
            })
        )
    }
}
