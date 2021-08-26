use nom::{bytes::complete::take_while1, combinator::map_res};
use std::str::FromStr;
use thiserror::Error;

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub type NomErr<'a> = nom::Err<nom::error::Error<&'a str>>;

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

pub fn take_uint<T: FromStr>(s: &str) -> IResult<T> {
    map_res(take_digits, |s| -> Result<T, <T as FromStr>::Err> {
        let n: T = s.parse()?;
        Ok(n)
    })(s)
}

fn take_digits(s: &str) -> IResult<&str> {
    take_while1(|c: char| c.is_ascii_digit())(s)
}
