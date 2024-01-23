use crate::{
    pos_error::PosError,
    shared::{text, SharedError},
};
use kanjidic_types::{kunyomi, Kunyomi};
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("(Kunyomi) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Kunyomi) Parsing: {0}, {1}")]
    Parse(PosError, kunyomi::ParseError),
}

pub fn from(node: Node) -> Result<Kunyomi, Error> {
    let text = text(&node)?;
    Kunyomi::try_from(text).map_err(|err| Error::Parse(PosError::from(&node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{kunyomi::KunyomiKind, Kunyomi};

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
                kind: KunyomiKind::Normal,
                reading: "つ".into(),
                okurigana: Some("ぐ".into()),
            })
        )
    }
}
