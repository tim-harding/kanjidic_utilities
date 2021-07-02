use std::{convert::TryFrom, str::FromStr};

mod database_version;
mod shared;

#[cfg(test)]
mod test_shared;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use database_version::DatabaseVersionError;
use nom::{
    bytes::complete::{take, take_till, take_while},
    character::{complete::char, is_digit},
    combinator::{eof, map, map_res},
    multi::many_till,
    sequence::tuple,
};
use roxmltree::Node;
use thiserror::Error;

use crate::database_version::DatabaseVersion;

#[derive(Debug, Error)]
pub enum KdpError {
    #[error("Error parsing XML file")]
    Xml(#[from] roxmltree::Error),
    #[error("Error parsing database version")]
    DatabaseVersion(#[from] DatabaseVersionError),
}

pub fn parse(xml: &str) -> Result<(), KdpError> {
    let doc = roxmltree::Document::parse(xml)?;
    Ok(())
}

struct Header {
    file_version: u8,
    database_version: DatabaseVersion,
    date_of_creation: DateOfCreation,
}

/*
impl<'a, 'b> TryFrom<Node<'a, 'b>> for Header {
    type Error = KdpError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {}
}
*/

struct DateOfCreation {
    year: u16,
    month: u8,
    date: u8,
}

/*
impl<'a, 'b> TryFrom<Node<'a, 'b>> for DateOfCreation {
    type Error = KdpError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {}
}
*/