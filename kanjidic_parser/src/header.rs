use crate::{
    database_version::{DatabaseVersion, DatabaseVersionError},
    date_of_creation::{DateOfCreation, DateOfCreationError},
    shared::{text_uint, SharedError},
};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

/// Error while parsing the header.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum HeaderError {
    #[error("(Header) Database version: {0}")]
    DatabaseVersion(#[from] DatabaseVersionError),
    #[error("(Header) Date of creation: {0}")]
    DateOfCreation(#[from] DateOfCreationError),
    #[error("(Header) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Header) Incomplete header provided")]
    Incomplete,
}

/// Contains identification information about the version of the file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Header {
    /// Denotes the version of the database structure.
    pub file_version: u8,
    /// The database version.
    pub database_version: DatabaseVersion,
    /// The date on which the database was created.
    pub date_of_creation: DateOfCreation,
}

struct HeaderBuilder {
    database_version: Option<DatabaseVersion>,
    date_of_creation: Option<DateOfCreation>,
    file_version: Option<u8>,
}

impl HeaderBuilder {
    pub fn new() -> Self {
        Self {
            database_version: None,
            date_of_creation: None,
            file_version: None,
        }
    }

    pub fn build(self) -> Result<Header, HeaderError> {
        let database_version = self.database_version.ok_or(HeaderError::Incomplete)?;
        let date_of_creation = self.date_of_creation.ok_or(HeaderError::Incomplete)?;
        let file_version = self.file_version.ok_or(HeaderError::Incomplete)?;

        Ok(Header {
            database_version,
            date_of_creation,
            file_version,
        })
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Header {
    type Error = HeaderError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut builder = HeaderBuilder::new();
        for child in node.children() {
            match child.tag_name().name() {
                "database_version" => {
                    builder.database_version = Some(DatabaseVersion::try_from(child)?);
                }
                "date_of_creation" => {
                    builder.date_of_creation = Some(DateOfCreation::try_from(child)?);
                }
                "file_version" => {
                    builder.file_version = Some(text_uint(&child)?);
                }
                _ => {},
            }
        }
        builder.build()
    }
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
                    day: 25,
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
