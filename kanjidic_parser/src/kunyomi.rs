use std::convert::TryFrom;

use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};
use kanjidic_types::{Kunyomi, KunyomiParseError};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KunyomiError {
    #[error("(Kunyomi) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Kunyomi) Parsing: {0}, {1}")]
    Parse(PosError, KunyomiParseError),
}

pub fn from(node: Node) -> Result<Kunyomi, KunyomiError> {
    let text = shared::text(&node)?;
    Kunyomi::try_from(text).map_err(|err| KunyomiError::Parse(PosError::from(&node), err))
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
