use crate::shared::{self, SharedError};
use kanjidic_types::{take_uint, IResult, NomErr, NomErrorReason};
use nom::{character::complete::char, combinator::map_res, sequence::tuple};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// The date the file was created
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DateOfCreation {
    /// Year of creation
    pub year: u16,
    /// Month of creation
    pub month: u8,
    /// Day of creation
    pub day: u8,
}

/// Error while parsing date of creation
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("(Date of creation) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Date of creation) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for Error {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for DateOfCreation {
    type Error = Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let text = shared::text(&node)?;
        Ok(map_res(take_db_version, map_db_version)(text).map(|(_, s)| s)?)
    }
}

type DateOfCreationParts<'a> = (u16, char, u8, char, u8);

fn take_db_version(s: &str) -> IResult<DateOfCreationParts> {
    tuple((take_uint, char('-'), take_uint, char('-'), take_uint))(s)
}

fn map_db_version(parts: DateOfCreationParts) -> Result<DateOfCreation, Error> {
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
