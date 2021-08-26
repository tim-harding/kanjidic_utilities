use std::convert::TryFrom;

use crate::{
    pos_error::PosError,
    shared::{text, SharedError},
};
use kanjidic_types::{Kuten, KutenParseError};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KutenError {
    #[error("(Kuten) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Kuten) Parsing: {0}, {1}")]
    Parse(PosError, KutenParseError),
}

pub fn from(node: Node) -> Result<Kuten, KutenError> {
    Kuten::try_from(text(node)?).map_err(|err| KutenError::Parse(PosError::from(node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::Kuten;

    #[test]
    fn parse_kuten() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("cp_value")
                    && node
                        .attribute("cp_type")
                        .map(|value| value.eq("jis208"))
                        .unwrap_or(false)
            })
            .unwrap();
        let kuten = from(node);
        assert_eq!(
            kuten,
            Ok(Kuten {
                plane: 1,
                ku: 16,
                ten: 1,
            })
        )
    }
}
