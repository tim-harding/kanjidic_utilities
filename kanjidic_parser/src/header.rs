use crate::{database_version::DatabaseVersion, date_of_creation::DateOfCreation};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HeaderError {}

pub struct Header {
    pub file_version: u8,
    pub database_version: DatabaseVersion,
    pub date_of_creation: DateOfCreation,
}

impl<'a, 'b> TryFrom<Node<'a, 'b>> for Header {
    type Error = HeaderError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        todo!()
    }
}
