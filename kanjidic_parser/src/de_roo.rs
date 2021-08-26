use crate::{
    pos_error::PosError,
    shared::{text, SharedError},
};
use kanjidic_types::{DeRoo, DeRooParseError};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq, Clone)]
pub enum DeRooError {
    #[error("(De Roo) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(De Roo) Parsing: {0}, {1}")]
    Str(PosError, DeRooParseError),
}

pub fn from(node: Node) -> Result<DeRoo, DeRooError> {
    DeRoo::try_from(text(node)?).map_err(|err| DeRooError::Str(PosError::from(node), err))
}

#[cfg(test)]
mod tests {
    use kanjidic_types::{DeRoo, ExtremeBottom, ExtremeTop};

    use super::from;
    use crate::test_shared::DOC;

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
