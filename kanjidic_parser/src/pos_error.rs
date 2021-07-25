use roxmltree::{Node, TextPos};
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PosError {
    position: TextPos,
}

impl fmt::Display for PosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location {}:{}", self.position.row, self.position.col)
    }
}

impl Error for PosError {}

impl<'a, 'input> From<Node<'a, 'input>> for PosError {
    fn from(node: Node<'a, 'input>) -> Self {
        let position = node.document().text_pos_at(node.range().start);
        Self { position }
    }
}
