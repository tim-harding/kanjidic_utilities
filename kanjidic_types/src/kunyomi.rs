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
#[serde(rename_all = "camelCase")]
pub struct Kunyomi {
    /// The kunyomi reading
    pub reading: String,
    /// The okurigana if relevant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okurigana: Option<String>,
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
pub enum KunyomiParseError {
    #[error("(Kunyomi) Format: {0}")]
    Format(NomErrorReason),
    #[error("(Kunyomi) Expected one or two kunyomi reading pieces")]
    IncorrectPieces,
}

impl<'a> From<NomErr<'a>> for KunyomiParseError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

impl TryFrom<&str> for Kunyomi {
    type Error = KunyomiParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, (pre, pieces, post)) = parts(text)?;
        let kind = if pre {
            KunyomiKind::Prefix
        } else if post {
            KunyomiKind::Suffix
        } else {
            KunyomiKind::Normal
        };
        let mut iter = pieces.into_iter();
        let reading = iter.next().ok_or(KunyomiParseError::IncorrectPieces)?;
        let okurigana = iter.next();
        if let Some(_) = iter.next() {
            return Err(KunyomiParseError::IncorrectPieces);
        }
        Ok(Kunyomi {
            reading,
            okurigana,
            kind,
        })
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
