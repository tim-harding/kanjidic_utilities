use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

use crate::shared::{digit, IResult, NomErrorReason};
use nom::{character::complete::char, combinator::map_res, sequence::tuple};

/// Error while parsing date of creation
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DateOfCreationError {
    /// No text in database version node
    #[error("No text in database version node")]
    NoText,

    /// Database version was not in a recognized format
    #[error("Database version was not in a recognized format")]
    Format(NomErrorReason),
}

/// The date the file was created
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateOfCreation {
    /// Year of creation
    pub year: u16,

    /// Month of creation
    pub month: u8,

    /// Day of creation
    pub day: u8,
}

impl<'a, 'b> TryFrom<Node<'a, 'b>> for DateOfCreation {
    type Error = DateOfCreationError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node.text() {
            Some(text) => map_res(take_db_version, map_db_version)(text)
                .map(|(_, s)| s)
                .map_err(|e| DateOfCreationError::Format(e.into())),
            None => Err(DateOfCreationError::NoText),
        }
    }
}

type DateOfCreationParts<'a> = (u16, char, u8, char, u8);

fn take_db_version(s: &str) -> IResult<DateOfCreationParts> {
    tuple((digit, char('-'), digit, char('-'), digit))(s)
}

fn map_db_version(parts: DateOfCreationParts) -> Result<DateOfCreation, DateOfCreationError> {
    let (year, _, month, _, date) = parts;
    Ok(DateOfCreation {
        year,
        month,
        day: date,
    })
}

#[cfg(test)]
mod tests {
    use crate::{date_of_creation::DateOfCreation, test_shared::DOC};
    use std::convert::TryFrom;

    #[test]
    fn gets_date_of_creation() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("date_of_creation"))
            .unwrap();
        let version = DateOfCreation::try_from(node);
        assert_eq!(
            version,
            Ok(DateOfCreation {
                year: 2021,
                month: 6,
                day: 25,
            })
        )
    }
}
