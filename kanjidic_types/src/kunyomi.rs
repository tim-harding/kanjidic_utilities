use crate::shared::{IResult, NomErr, NomErrorReason};
use nom::{
    bytes::complete::is_not,
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::tuple,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

/// A kunyomi kanji reading.
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Kunyomi {
    /// The okurigana
    pub okurigana: Vec<String>,
    /// Whether the reading is as a prefix or suffix.
    pub kind: KunyomiKind,
}

/// The kind of kunyomi reading.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum KunyomiKind {
    /// A normal reading
    Normal,
    /// A prefix
    Prefix,
    /// A suffix
    Suffix,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KunyomiStrError {
    #[error("(Kunyomi) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for KunyomiStrError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

impl TryFrom<&str> for Kunyomi {
    type Error = KunyomiStrError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, (pre, okurigana, post)) = parts(text)?;
        let kind = if pre {
            KunyomiKind::Prefix
        } else if post {
            KunyomiKind::Suffix
        } else {
            KunyomiKind::Normal
        };
        Ok(Kunyomi { okurigana, kind })
    }
}

fn parts(s: &str) -> IResult<(bool, Vec<String>, bool)> {
    tuple((fix, okurigana, fix))(s)
}

fn okurigana(s: &str) -> IResult<Vec<String>> {
    separated_list1(char('.'), map(is_not("-."), |s: &str| s.into()))(s)
}

fn fix(s: &str) -> IResult<bool> {
    map(opt(char('-')), |c| c.is_some())(s)
}
