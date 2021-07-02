use crate::{
    database_version::{DatabaseVersion, DatabaseVersionError},
    date_of_creation::{DateOfCreation, DateOfCreationError},
    file_version::{FileVersion, FileVersionError},
};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HeaderError {
    #[error("Error parsing database version")]
    DatabaseVersion(#[from] DatabaseVersionError),
    #[error("Error parsing file version")]
    DateOfCreation(#[from] DateOfCreationError),
    #[error("There was a problem parsing the file version")]
    FileVersion(#[from] FileVersionError),
    #[error("Header is missing tag {0}")]
    MissingTag(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub file_version: FileVersion,
    pub database_version: DatabaseVersion,
    pub date_of_creation: DateOfCreation,
}

impl<'a, 'b> TryFrom<Node<'a, 'b>> for Header {
    type Error = HeaderError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let database_version = DatabaseVersion::try_from(
            node.descendants()
                .find(|child| child.has_tag_name("database_version"))
                .ok_or(HeaderError::MissingTag("database_version"))?,
        )?;
        let date_of_creation = DateOfCreation::try_from(
            node.descendants()
                .find(|child| child.has_tag_name("date_of_creation"))
                .ok_or(HeaderError::MissingTag("date_of_creation"))?,
        )?;
        let file_version = FileVersion::try_from(
            node.descendants()
                .find(|child| child.has_tag_name("file_version"))
                .ok_or(HeaderError::MissingTag("file_version"))?,
        )?;
        Ok(Header {
            database_version,
            date_of_creation,
            file_version,
        })
    }
}

/*
fn from_descendant<'a, 'input, T>(
    root: Node<'a, 'input>,
    tag: &'static str,
) -> Result<T, HeaderError>
where
    T: TryFrom<Node<'a, 'input>, Error = Into<HeaderError>>,
{
    T::try_from(
        root.descendants()
            .find(|child| child.has_tag_name(tag))
            .ok_or(HeaderError::MissingTag(tag))?,
    )
}
*/

#[cfg(test)]
mod tests {
    use super::Header;
    use crate::{
        database_version::DatabaseVersion, date_of_creation::DateOfCreation,
        file_version::FileVersion, test_shared::DOC,
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
                file_version: FileVersion(4),
            })
        )
    }
}
