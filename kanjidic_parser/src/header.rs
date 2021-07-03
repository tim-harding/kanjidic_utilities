use crate::{
    database_version::{DatabaseVersion, DatabaseVersionError},
    date_of_creation::{DateOfCreation, DateOfCreationError},
    node_number::{node_number, NodeNumberError},
    shared::{descendant, SharedError},
};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

/// Error while parsing the header.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum HeaderError {
    /// Error while parsing the database version.
    #[error("Error parsing database version")]
    DatabaseVersion(#[from] DatabaseVersionError),

    /// Error while parsing the date of creation.
    #[error("Error parsing file version")]
    DateOfCreation(#[from] DateOfCreationError),

    /// Error while parsing the file version.
    #[error("There was a problem parsing the file version")]
    FileVersion(#[from] NodeNumberError),

    /// The header tag was not found.
    #[error("Header is missing tag {0}")]
    MissingTag(#[from] SharedError),
}

/// Contains identification information about the version of the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// Denotes the version of the database structure.
    pub file_version: u8,

    /// The database version.
    pub database_version: DatabaseVersion,

    /// The date on which the database was created.
    pub date_of_creation: DateOfCreation,
}

impl<'a, 'b> TryFrom<Node<'a, 'b>> for Header {
    type Error = HeaderError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let database_version = DatabaseVersion::try_from(descendant(node, "database_version")?)?;
        let date_of_creation = DateOfCreation::try_from(descendant(node, "date_of_creation")?)?;
        let file_version = node_number(descendant(node, "file_version")?)?;
        Ok(Header {
            database_version,
            date_of_creation,
            file_version,
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
