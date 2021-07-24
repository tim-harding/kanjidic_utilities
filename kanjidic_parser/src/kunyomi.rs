use crate::shared::IResult;
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

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KunyomiError {
    #[error("Node contains no text")]
    NoText,
    #[error("Kunyomi format not recognized")]
    Format,
}

/// A kunyomi kanji reading.
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Kunyomi<'a> {
    /// The okurigana
    pub okurigana: Vec<&'a str>,

    /// Whether the reading is as a prefix or suffix.
    pub kind: KunyomiKind,
}

/// The kind of kunyomi reading.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum KunyomiKind {
    /// A normal reading
    Normal,

    /// A prefix
    Prefix,

    /// A suffix
    Suffix,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Kunyomi<'a> {
    type Error = KunyomiError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = node.text().ok_or(KunyomiError::NoText)?;
        let (_i, (pre, okurigana, post)) = parts(text).map_err(|_| KunyomiError::Format)?;
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
