use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

use crate::shared::{IResult, NomErrorReason};
use nom::{
    bytes::complete::take_while, character::complete::char, combinator::map_res, sequence::tuple,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DateOfCreationError {
    #[error("No text in database version node")]
    NoText,
    #[error("Database version was not in a recognized format")]
    Format(NomErrorReason),
    #[error("Could not parse an integer")]
    Integer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateOfCreation {
    pub year: u16,
    pub month: u8,
    pub date: u8,
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

type DateOfCreationParts<'a> = (&'a str, char, &'a str, char, &'a str);

fn take_db_version(s: &str) -> IResult<DateOfCreationParts> {
    tuple((
        take_while(|c: char| c.is_ascii_digit()),
        char('-'),
        take_while(|c: char| c.is_ascii_digit()),
        char('-'),
        take_while(|c: char| c.is_ascii_digit()),
    ))(s)
}

fn map_db_version(parts: DateOfCreationParts) -> Result<DateOfCreation, DateOfCreationError> {
    let (year, _, month, _, date) = parts;
    let year: u16 = year.parse().map_err(|_| DateOfCreationError::Integer)?;
    let month: u8 = month.parse().map_err(|_| DateOfCreationError::Integer)?;
    let date: u8 = date.parse().map_err(|_| DateOfCreationError::Integer)?;
    Ok(DateOfCreation { year, month, date })
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
                date: 25,
            })
        )
    }
}
