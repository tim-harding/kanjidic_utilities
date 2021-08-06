use crate::{
    pos_error::PosError,
    shared::{self, IResult, NomErr, NomErrorReason, SharedError},
};
use kanjidic_types::{Kunyomi, KunyomiKind};
use nom::{
    bytes::complete::is_not,
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::tuple,
};
use roxmltree::Node;
use thiserror::Error;

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

pub fn from(node: Node) -> Result<Kunyomi, KunyomiError> {
    let text = shared::text(node)?;
    from_str(text).map_err(|err| KunyomiError::Parse(PosError::from(node), err))
}

fn from_str(text: &str) -> Result<Kunyomi, KunyomiStrError> {
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

fn parts(s: &str) -> IResult<(bool, Vec<String>, bool)> {
    tuple((fix, okurigana, fix))(s)
}

fn okurigana(s: &str) -> IResult<Vec<String>> {
    separated_list1(char('.'), map(is_not("-."), |s: &str| s.into()))(s)
}

fn fix(s: &str) -> IResult<bool> {
    map(opt(char('-')), |c| c.is_some())(s)
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{Kunyomi, KunyomiKind};

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
        let kunyomi = from(node);
        assert_eq!(
            kunyomi,
            Ok(Kunyomi {
                okurigana: vec!["つ".into(), "ぐ".into(),],
                kind: KunyomiKind::Normal,
            })
        )
    }
}
