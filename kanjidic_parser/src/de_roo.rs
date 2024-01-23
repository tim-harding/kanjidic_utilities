use crate::{
    pos_error::PosError,
    shared::{text, SharedError},
};
use kanjidic_types::{de_roo, DeRoo};
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Debug, thiserror::Error, Eq, PartialEq, Clone)]
pub enum Error {
    #[error("(De Roo) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(De Roo) Parsing: {0}, {1}")]
    Str(PosError, de_roo::ParseError),
}

pub fn from(node: Node) -> Result<DeRoo, Error> {
    DeRoo::try_from(text(&node)?).map_err(|err| Error::Str(PosError::from(&node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{
        de_roo::{ExtremeBottom, ExtremeTop},
        DeRoo,
    };

    #[test]
    fn de_roo() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("q_code")
                    && node
                        .attribute("qc_type")
                        .map(|value| value.eq("deroo"))
                        .unwrap_or(false)
            })
            .unwrap();
        let deroo = from(node);
        assert_eq!(
            deroo,
            Ok(DeRoo {
                top: ExtremeTop::Bald,
                bottom: ExtremeBottom::StandingBottom,
            })
        )
    }
}
