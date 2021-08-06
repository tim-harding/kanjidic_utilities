use crate::{
    database_version::{DatabaseVersion, DatabaseVersionError},
    date_of_creation::{DateOfCreation, DateOfCreationError},
    shared::{child, text_uint, SharedError},
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

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Header {
    type Error = HeaderError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let database_version = DatabaseVersion::try_from(child(node, "database_version")?)?;
        let date_of_creation = DateOfCreation::try_from(child(node, "date_of_creation")?)?;
        let file_version = text_uint(child(node, "file_version")?)?;
        Ok(Header {
            file_version,
            database_version,
            date_of_creation,
        })
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
