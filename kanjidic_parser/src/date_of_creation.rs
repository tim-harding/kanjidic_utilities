use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DateOfCreationError {}

pub struct DateOfCreation {
    pub year: u16,
    pub month: u8,
    pub date: u8,
}

impl<'a, 'b> TryFrom<Node<'a, 'b>> for DateOfCreation {
    type Error = DateOfCreationError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        todo!()
    }
}
