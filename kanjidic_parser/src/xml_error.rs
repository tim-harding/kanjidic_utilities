use roxmltree::{Node, TextPos};
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlError<'a> {
    region: &'a str,
    position: TextPos,
}

impl<'a> fmt::Display for XmlError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at {}:{}: {}",
            self.position.row, self.position.col, self.region
        )
    }
}

impl<'a> Error for XmlError<'a> {}

impl<'a, 'input> From<Node<'a, 'input>> for XmlError<'a> {
    fn from(node: Node<'a, 'input>) -> Self {
        let range = node.range();
        let doc = node.document();
        let text = doc.input_text();
        let byte_region = &text.as_bytes()[range.clone()];
        let region = std::str::from_utf8(byte_region).unwrap_or("Could not produce a region");
        let position = doc.text_pos_at(range.start);
        Self { region, position }
    }
}
