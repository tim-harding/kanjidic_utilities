use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};
use kanjidic_types::{pin_yin, PinYin};
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("(Pin Yin) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Pin Yin) Parsing: {0}, {1}")]
    Parse(PosError, pin_yin::ParseError),
}

pub fn from(node: Node) -> Result<PinYin, Error> {
    let text = shared::text(&node)?;
    PinYin::try_from(text).map_err(|err| Error::Parse(PosError::from(&node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{pin_yin::Tone, PinYin};

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
