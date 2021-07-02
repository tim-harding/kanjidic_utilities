use crate::shared::IResult;
use nom::{
    bytes::complete::{take, take_while},
    character::complete::char,
    combinator::map_res,
    sequence::tuple,
};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DatabaseVersionError {
    #[error("No text in database version node")]
    NoText,
    #[error("Database version was not in a recognized format")]
    Format,
    #[error("Could not parse an integer")]
    Integer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{database_version::DatabaseVersion, test_shared::DOC};

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
