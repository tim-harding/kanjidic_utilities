use std::str::FromStr;

use nom::{bytes::complete::take_while1, combinator::map_res};
use roxmltree::Node;
use thiserror::Error;

use crate::pos_error::PosError;

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub type NomErr<'a> = nom::Err<nom::error::Error<&'a str>>;

// Todo: Make sure all error display strings are good

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NomErrorReason {
    #[error("(Nom) incomplete")]
    Incomplete,
    #[error("(Nom) error kind: {0:?}")]
    Error(nom::error::ErrorKind),
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

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SharedError {
    #[error("(Shared) No node with the given tag: {0}, attribute '{1}'")]
    MissingChild(PosError, &'static str),
    #[error("(Shared) Node contains no text")]
    NoText(PosError),
    #[error("(Shared) Could not parse text as a uint")]
    TextUint(PosError),
    #[error("(Shared) Could not parse attribute as a uint")]
    AttrUint(PosError),
    #[error("(Shared) Node missing attribute: {0}, attribute '{1}'")]
    MissingAttribute(PosError, &'static str),
    #[error("(Shared) Could not parse hexadecimal")]
    Hex(PosError),
}

pub fn child<'a, 'input>(
    node: Node<'a, 'input>,
    tag: &'static str,
) -> Result<Node<'a, 'input>, SharedError> {
    node.children()
        .find(|child| child.has_tag_name(tag))
        .ok_or_else(|| SharedError::MissingChild(PosError::from(node), tag))
}

pub fn children<'a, 'input, T, E, F>(
    node: Node<'a, 'input>,
    tag: &'static str,
    cb: F,
) -> Result<Vec<T>, E>
where
    E: std::error::Error,
    F: Fn(Node<'a, 'input>) -> Result<T, E>,
{
    node.children()
        .filter(|child| child.has_tag_name(tag))
        .map(cb)
        .collect()
}

fn take_digits(s: &str) -> IResult<&str> {
    take_while1(|c: char| c.is_ascii_digit())(s)
}

pub fn take_uint<T: FromStr>(s: &str) -> IResult<T> {
    map_res(take_digits, |s| -> Result<T, <T as FromStr>::Err> {
        let n: T = s.parse()?;
        Ok(n)
    })(s)
}

pub fn text_uint<T: FromStr>(node: Node) -> Result<T, SharedError> {
    text(node)?
        .parse::<T>()
        .map_err(|_| SharedError::TextUint(PosError::from(node)))
}

pub fn text_hex(node: Node) -> Result<u32, SharedError> {
    let text = text(node)?;
    u32::from_str_radix(&text, 16).map_err(|_| SharedError::Hex(PosError::from(node)))
}

pub fn text<'a, 'input>(node: Node<'a, 'input>) -> Result<&'a str, SharedError> {
    node.text()
        .ok_or_else(|| SharedError::NoText(PosError::from(node)))
}

pub fn attr<'a, 'input>(
    node: Node<'a, 'input>,
    attribute: &'static str,
) -> Result<&'a str, SharedError> {
    node.attribute(attribute)
        .ok_or_else(|| SharedError::MissingAttribute(PosError::from(node), attribute))
}

pub fn attr_uint<T: FromStr>(
    node: Node,
    attribute: &'static str,
) -> Result<Option<T>, SharedError> {
    match node.attribute(attribute) {
        Some(text) => {
            let parsed: T = text
                .parse()
                .map_err(|_| SharedError::AttrUint(PosError::from(node)))?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}
