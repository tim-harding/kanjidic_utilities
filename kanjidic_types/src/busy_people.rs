use std::convert::TryFrom;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::char,
    combinator::{map, map_res, value},
    sequence::tuple,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{IResult, NomErr, NomErrorReason};

/// A location in Japanese for Busy People.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusyPeople {
    /// The volume
    pub volume: u8,
    /// The chapter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<u8>,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum BusyPeopleParseError {
    #[error("(Busy people) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for BusyPeopleParseError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

impl TryFrom<&str> for BusyPeople {
    type Error = BusyPeopleParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, o) = parts(text)?;
        let (volume, _, chapter) = o;
        Ok(Self { volume, chapter })
    }
}

fn parts(s: &str) -> IResult<(u8, char, Option<u8>)> {
    tuple((number, char('.'), chapter))(s)
}

fn chapter(s: &str) -> IResult<Option<u8>> {
    alt((value(None, char('A')), map(number, Some)))(s)
}

fn number(s: &str) -> IResult<u8> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u8>()
    })(s)
}
