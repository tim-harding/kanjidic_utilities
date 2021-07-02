use crate::{
    database_version::{DatabaseVersion, DatabaseVersionError},
    date_of_creation::{DateOfCreation, DateOfCreationError},
    shared::NomErr,
};
use nom::{bytes::complete::take_while1, combinator::map_res};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HeaderError {
    #[error("No database version tag")]
    NoDatabaseVersionTag,
    #[error("Error parsing database version")]
    DatabaseVersion(#[from] DatabaseVersionError),
    #[error("No date of creation tag")]
    NoDateOfCreationTag,
    #[error("Error parsing file version")]
    DateOfCreation(#[from] DateOfCreationError),
    #[error("No file version tag")]
    NoFileVersionTag,
    #[error("No file version text")]
    NoFileVersionText,
    #[error("There was a problem parsing the file version")]
    FileVersion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub file_version: u8,
    pub database_version: DatabaseVersion,
    pub date_of_creation: DateOfCreation,
}

impl<'a, 'b> TryFrom<Node<'a, 'b>> for Header {
    type Error = HeaderError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let database_version = DatabaseVersion::try_from(
            node.descendants()
                .find(|child| child.has_tag_name("database_version"))
                .ok_or(HeaderError::NoDatabaseVersionTag)?,
        )?;
        let date_of_creation = DateOfCreation::try_from(
            node.descendants()
                .find(|child| child.has_tag_name("date_of_creation"))
                .ok_or(HeaderError::NoDateOfCreationTag)?,
        )?;
        let file_version_node = node
            .descendants()
            .find(|child| child.has_tag_name("file_version"))
            .ok_or(HeaderError::NoFileVersionTag)?;
        let file_version_text = file_version_node
            .text()
            .ok_or(HeaderError::NoFileVersionText)?;
        let file_version = parse_file_version(file_version_text)?;
        Ok(Header {
            database_version,
            date_of_creation,
            file_version,
        })
    }
}

fn parse_file_version(s: &str) -> Result<u8, HeaderError> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), map_file_version)(s)
        .map(|(_, v)| v)
        .map_err(|_e: NomErr| HeaderError::FileVersion)
}

fn map_file_version(n: &str) -> Result<u8, HeaderError> {
    let version: u8 = n.parse().map_err(|_| HeaderError::FileVersion)?;
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::Header;
    use crate::{
        database_version::DatabaseVersion, date_of_creation::DateOfCreation, test_shared::DOC,
    };
    use std::convert::TryFrom;

    #[test]
    fn parses_header() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("header"))
            .unwrap();
        let header = Header::try_from(node);
        assert_eq!(
            header,
            Ok(Header {
                date_of_creation: DateOfCreation {
                    year: 2021,
                    month: 6,
                    date: 25,
                },
                database_version: DatabaseVersion {
                    year: 2021,
                    version: 176,
                },
                file_version: 4,
            })
        )
    }
}
