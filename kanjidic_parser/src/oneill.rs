use crate::{
    pos_error::PosError,
    shared::{take_uint, text, IResult, NomErr, NomErrorReason, SharedError},
};
use kanjidic_types::{Oneill, OneillSuffix};
use nom::{
    bytes::complete::take_while,
    combinator::{map, map_res},
    sequence::tuple,
};
use roxmltree::Node;
use thiserror::Error;

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

pub fn from(node: Node) -> Result<Oneill, OneillError> {
    from_str(text(node)?).map_err(|err| OneillError::Parse(PosError::from(node), err))
}

fn from_str(text: &str) -> Result<Oneill, OneillStrError> {
    let (_i, index) = parse(text)?;
    Ok(index)
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
