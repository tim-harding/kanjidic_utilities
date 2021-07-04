use std::str::FromStr;

use nom::{bytes::complete::take_while1, combinator::map_res};
use roxmltree::Node;
use thiserror::Error;

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub type NomErr<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NomErrorReason {
    Incomplete,
    Error(nom::error::ErrorKind),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SharedError {
    #[error("Could not find a node with the given tag")]
    MissingTag(&'static str),
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

pub fn digit<T: FromStr>(s: &str) -> IResult<T> {
    map_res(take_digits, |s| -> Result<T, <T as FromStr>::Err> {
        let n: T = s.parse()?;
        Ok(n)
    })(s)
}
