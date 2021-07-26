use nom::{
    bytes::complete::take_while,
    combinator::{map, map_res},
    sequence::tuple,
};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::{
    pos_error::PosError,
    shared::{take_uint, text, IResult, NomErr, NomErrorReason, SharedError},
};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OneillError {
    #[error("(Oneill) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Oneill) Parsing: {0}, {1}")]
    Parse(PosError, OneillStrError),
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

/// An index into the Japanese Names reference book
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,Serialize, Deserialize)]
pub struct Oneill {
    /// The reference number
    pub number: u16,
    /// A reference's suffix
    pub suffix: OneillSuffix,
}

/// The suffix for a Japanese Names reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash , Serialize, Deserialize)]
pub enum OneillSuffix {
    /// No suffix
    None,
    /// 'A' suffix
    A,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Oneill {
    type Error = OneillError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        Self::try_from(text(node)?).map_err(|err| OneillError::Parse(PosError::from(node), err))
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
