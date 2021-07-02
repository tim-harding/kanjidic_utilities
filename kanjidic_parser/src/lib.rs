use std::{convert::TryFrom, str::FromStr};

use nom::{
    bytes::complete::{take, take_till, take_while},
    character::{complete::char, is_digit},
    combinator::{eof, map, map_res},
    multi::many_till,
    sequence::tuple,
};
use roxmltree::Node;
use thiserror::Error;

type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug, Error)]
pub enum KdpError {
    #[error("Error parsing XML file")]
    Xml(#[from] roxmltree::Error),
    #[error("Error parsing database version")]
    DatabaseVersion(#[from] DatabaseVersionError),
}

pub fn parse(xml: &str) -> Result<(), KdpError> {
    let doc = roxmltree::Document::parse(xml)?;
    let node = doc.descendants().find(|node| node.has_tag_name("database_version")).unwrap();
    let version = DatabaseVersion::try_from(node);
    println!("{:?}" , version);
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

#[derive(Debug, Error)]
pub enum DatabaseVersionError {
    #[error("No text in database version node")]
    NoText,
    #[error("Database version was not in a recognized format")]
    Format,
    #[error("Could not parse an integer")]
    Integer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DatabaseVersion {
    pub year: u16,
    pub version: u16,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for DatabaseVersion {
    type Error = DatabaseVersionError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node.text() {
            Some(text) => match map_res(take_db_version, map_db_version)(text) {
                Ok((_, s)) => Ok(s),
                Err(_) => Err(DatabaseVersionError::Format),
            },
            None => Err(DatabaseVersionError::NoText),
        }
    }
}

type DbVersionParts<'a> = (&'a str, char, &'a str);

fn take_db_version(s: &str) -> IResult<DbVersionParts> {
    tuple((
        take(4u8),
        char('-'),
        take_while(|c: char| c.is_ascii_digit()),
    ))(s)
}

fn map_db_version(parts: DbVersionParts) -> Result<DatabaseVersion, DatabaseVersionError> {
    let (year, _, version) = parts;
    let year: u16 = year.parse().map_err(|_| DatabaseVersionError::Integer)?;
    let version: u16 = version.parse().map_err(|_| DatabaseVersionError::Integer)?;
    Ok(DatabaseVersion { year, version })
}
