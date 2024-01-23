use crate::{
    pos_error::PosError,
    shared::{text_uint, SharedError},
};
use kanjidic_types::StrokeCount;
use roxmltree::Node;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
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
    pub fn add_from_node(&mut self, node: &Node) -> Result<(), Error> {
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

    pub fn build(self) -> Result<StrokeCount, Error> {
        let accepted = self.accepted.ok_or(Error::Incomplete)?;
        Ok(StrokeCount {
            accepted,
            miscounts: self.miscounts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use crate::{pos_error::PosError, shared::text_uint, test_shared::DOC};
    use kanjidic_types::StrokeCount;
    use roxmltree::Node;

    // TODO: Refactor to use StrokeCountBuilder
    pub fn from(node: &Node) -> Result<StrokeCount, Error> {
        let mut children = node
            .children()
            .filter(|child| child.has_tag_name("stroke_count"))
            .map(|child| Ok(text_uint(&child)?));
        let accepted = children
            .next()
            .ok_or_else(|| Error::Accepted(PosError::from(node)))??;
        let miscounts: Result<Vec<u8>, Error> = children.collect();
        let miscounts = miscounts?;
        Ok(StrokeCount {
            accepted,
            miscounts,
        })
    }

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
