use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};
use kanjidic_types::{PinYin, PinYinParseError};

use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum PinYinError {
    #[error("(Pin Yin) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Pin Yin) Parsing: {0}, {1}")]
    Parse(PosError, PinYinParseError),
}

pub fn from(node: Node) -> Result<PinYin, PinYinError> {
    let text = shared::text(&node)?;
    PinYin::try_from(text).map_err(|err| PinYinError::Parse(PosError::from(&node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{PinYin, Tone};

    #[test]
    fn pin_yin() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("reading")
                    && node
                        .attribute("r_type")
                        .map(|value| value.eq("pinyin"))
                        .unwrap_or(false)
            })
            .unwrap();
        let pin_yin = from(node);
        assert_eq!(
            pin_yin,
            Ok(PinYin {
                romanization: "ya".into(),
                tone: Tone::Falling,
            })
        )
    }
}
