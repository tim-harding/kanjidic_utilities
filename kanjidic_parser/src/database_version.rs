use crate::shared::{self, SharedError};
use kanjidic_types::{take_uint, IResult, NomErr, NomErrorReason};
use nom::{character::complete::char, combinator::map_res, sequence::tuple};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

/// Error while parsing the database version
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DatabaseVersionError {
    #[error("(Database version) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Database version) Format: {0}")]
    Format(NomErrorReason),
}

/// The version of the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DatabaseVersion {
    /// The year of release.
    pub year: u16,
    /// The version that came out in the given year,
    /// with the counter being reset annually.
    pub version: u16,
}

impl<'a> From<NomErr<'a>> for DatabaseVersionError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for DatabaseVersion {
    type Error = DatabaseVersionError;

    fn try_from(node: Node) -> Result<DatabaseVersion, DatabaseVersionError> {
        let text = shared::text(&node)?;
        Ok(map_res(take_db_version, map_db_version)(text).map(|(_, s)| s)?)
    }
}

type DbVersionParts<'a> = (u16, char, u16);

fn take_db_version(s: &str) -> IResult<DbVersionParts> {
    tuple((take_uint, char('-'), take_uint))(s)
}

fn map_db_version(parts: DbVersionParts) -> Result<DatabaseVersion, DatabaseVersionError> {
    let (year, _, version) = parts;
    Ok(DatabaseVersion { year, version })
}

#[cfg(test)]
mod tests {
    use crate::{test_shared::DOC, DatabaseVersion};
    use std::convert::TryFrom;

    #[test]
    fn gets_db_version() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("database_version"))
            .unwrap();
        let version = DatabaseVersion::try_from(node);
        assert_eq!(
            version,
            Ok(DatabaseVersion {
                year: 2021,
                version: 176,
            })
        )
    }
}
