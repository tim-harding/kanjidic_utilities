use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::char,
    combinator::{map, map_res, value},
    sequence::tuple,
};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

use crate::shared::IResult;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum BusyPeopleError {
    #[error("Node contains no text")]
    NoText,
    #[error("Unrecognized chapter format")]
    Parse,
}

/// A location in Japanese for Busy People.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BusyPeople {
    /// The volume
    volume: u8,

    /// The chapter
    chapter: Chapter,
}

/// Either the chapter number or chapter A in Japanese for Busy People.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Chapter {
    /// A chapter number.
    Chapter(u8),

    /// Some of the chapter are called "A",
    /// but it isn't specified anywhere what that means.
    A,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for BusyPeople {
    type Error = BusyPeopleError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = node.text().ok_or(BusyPeopleError::NoText)?;
        BusyPeople::try_from(text)
    }
}

impl<'a> TryFrom<&'a str> for BusyPeople {
    type Error = BusyPeopleError;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let (_i, o) = parts(text).map_err(|_| BusyPeopleError::Parse)?;
        let (volume, _, chapter) = o;
        Ok(BusyPeople { volume, chapter })
    }
}

fn parts(s: &str) -> IResult<(u8, char, Chapter)> {
    tuple((number, char('.'), chapter))(s)
}

fn chapter(s: &str) -> IResult<Chapter> {
    alt((
        value(Chapter::A, char('a')),
        map(number, |n| Chapter::Chapter(n)),
    ))(s)
}

fn number(s: &str) -> IResult<u8> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u8>()
    })(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn busy_people() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("dic_ref")
                    && node
                        .attribute("dr_type")
                        .map(|value| value.eq("busy_people"))
                        .unwrap_or(false)
            })
            .unwrap();
        let busy_people = BusyPeople::try_from(node);
        assert_eq!(
            busy_people,
            Ok(BusyPeople {
                volume: 3,
                chapter: Chapter::Chapter(14),
            })
        )
    }
}
