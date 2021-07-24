use std::str::FromStr;

use nom::{bytes::complete::take_while1, combinator::map_res};
use roxmltree::Node;
use thiserror::Error;

use crate::pos_error::PosError;

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub type NomErr<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NomErrorReason {
    Incomplete,
    Error(nom::error::ErrorKind),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SharedError {
    #[error("Could not find a node with the given tag: {0}")]
    MissingTag(&'static str),
    #[error("Node contains no text")]
    NoText(PosError),
    #[error("Could not parse text as a uint")]
    Numeric(PosError),
    #[error("Missing node attribute: {0}, attribute '{1}'")]
    MissingAttribute(PosError, &'static str),
}

impl<'a> From<NomErr<'a>> for NomErrorReason {
    fn from(err: NomErr) -> Self {
        use nom::Err::*;

        match err {
            Incomplete(_) => NomErrorReason::Incomplete,
            Error(e) | Failure(e) => NomErrorReason::Error(e.code),
        }
    }
}

pub fn descendant<'a, 'input>(
    node: Node<'a, 'input>,
    tag: &'static str,
) -> Result<Node<'a, 'input>, SharedError> {
    node.descendants()
        .find(|child| child.has_tag_name(tag))
        .ok_or(SharedError::MissingTag(tag))
}

fn take_digits(s: &str) -> IResult<&str> {
    take_while1(|c: char| c.is_ascii_digit())(s)
}

pub fn uint<T: FromStr>(s: &str) -> IResult<T> {
    map_res(take_digits, |s| -> Result<T, <T as FromStr>::Err> {
        let n: T = s.parse()?;
        Ok(n)
    })(s)
}

pub fn numeric_code<T: FromStr>(node: Node) -> Result<T, SharedError> {
    text(node)?
        .parse::<T>()
        .map_err(|_| SharedError::Numeric(PosError::from(node)))
}

pub fn text<'a, 'input>(node: Node<'a, 'input>) -> Result<&'a str, SharedError> {
    node.text().ok_or(SharedError::NoText(PosError::from(node)))
}

pub fn attr<'a, 'input>(
    node: Node<'a, 'input>,
    attribute: &'static str,
) -> Result<&'a str, SharedError> {
    node.attribute(attribute)
        .ok_or(SharedError::MissingAttribute(
            PosError::from(node),
            attribute,
        ))
}
