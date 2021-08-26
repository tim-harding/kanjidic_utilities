use crate::{shared::NomErrorReason, take_uint, IResult, NomErr};
use nom::{
    bytes::complete::take_while,
    combinator::{map, map_res},
    sequence::tuple,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

/// An index into the Japanese Names reference book
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Oneill {
    /// The reference number
    pub number: u16,
    /// A reference's suffix
    pub suffix: OneillSuffix,
}

/// The suffix for a Japanese Names reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OneillSuffix {
    /// No suffix
    None,
    /// 'A' suffix
    A,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OneillStrError {
    #[error("(Oneill) Unknown reference suffix")]
    UnknownSuffix,
    #[error("(Oneill) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for OneillStrError {
    fn from(err: NomErr<'a>) -> Self {
        OneillStrError::Format(err.into())
    }
}

impl TryFrom<&str> for Oneill {
    type Error = OneillStrError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, index) = parse(text)?;
        Ok(index)
    }
}

fn parse(s: &str) -> IResult<Oneill> {
    map(parts, |parts| {
        let (number, suffix) = parts;
        Oneill { number, suffix }
    })(s)
}

fn parts(s: &str) -> IResult<(u16, OneillSuffix)> {
    tuple((take_uint, suffix))(s)
}

fn suffix(s: &str) -> IResult<OneillSuffix> {
    map_res(take_while(|c: char| c.is_ascii_alphabetic()), |v| match v {
        "A" => Ok(OneillSuffix::A),
        "" => Ok(OneillSuffix::None),
        _ => Err(OneillStrError::UnknownSuffix),
    })(s)
}
