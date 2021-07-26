use crate::{
    pos_error::PosError,
    shared::{self, IResult, NomErr, NomErrorReason, SharedError},
};
use nom::{
    bytes::complete::is_not,
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::tuple,
};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;
use kanjidic_types::{Kunyomi, KunyomiKind};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KunyomiError {
    #[error("(Kunyomi) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Kunyomi) Parsing: {0}, {1}")]
    Parse(PosError, KunyomiStrError),
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

/// A kunyomi kanji reading.
impl<'a, 'b: 'a> TryFrom<&'b str> for Kunyomi<'a> {
    type Error = KunyomiStrError;

    fn try_from(text: &'b str) -> Result<Self, Self::Error> {
        let (_i, (pre, okurigana, post)) = parts(text)?;
        let kind = if pre {
            KunyomiKind::Prefix
        } else if post {
            KunyomiKind::Suffix
        } else {
            KunyomiKind::Normal
        };
        Ok(Self { okurigana, kind })
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Kunyomi<'a> {
    type Error = KunyomiError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = shared::text(node)?;
        Self::try_from(text).map_err(|err| KunyomiError::Parse(PosError::from(node), err))
    }
}

fn parts(s: &str) -> IResult<(bool, Vec<&str>, bool)> {
    tuple((fix, okurigana, fix))(s)
}

fn okurigana(s: &str) -> IResult<Vec<&str>> {
    separated_list1(char('.'), is_not("-."))(s)
}

fn fix(s: &str) -> IResult<bool> {
    map(opt(char('-')), |c| c.is_some())(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn kunyomi() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("reading")
                    && node
                        .attribute("r_type")
                        .map(|value| value.eq("ja_kun"))
                        .unwrap_or(false)
            })
            .unwrap();
        let kunyomi = Kunyomi::try_from(node);
        assert_eq!(
            kunyomi,
            Ok(Kunyomi {
                okurigana: vec!["つ", "ぐ",],
                kind: KunyomiKind::Normal,
            })
        )
    }
}
