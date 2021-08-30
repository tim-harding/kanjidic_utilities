use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};
use kanjidic_types::{ShDesc, ShParseError};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ShError {
    #[error("(Spahn Hadamitzky) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Spahn Hadamitzky) Parsing: {0}, {1}")]
    Parse(PosError, ShParseError),
}

pub fn from(node: Node) -> Result<ShDesc, ShError> {
    let text = shared::text(node)?;
    ShDesc::try_from(text).map_err(|err| ShError::Parse(PosError::from(node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::ShDesc;

    #[test]
    fn spahn_hadamitzky() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("variant")
                    && node
                        .attribute("var_type")
                        .map(|value| value.eq("s_h"))
                        .unwrap_or(false)
            })
            .unwrap();
        let sh = from(node);
        assert_eq!(
            sh,
            Ok(ShDesc {
                radical_strokes: 2,
                radical: 'k',
                other_strokes: 4,
                sequence: 6,
            })
        )
    }
}
