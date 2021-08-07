use crate::{
    kunyomi, pin_yin,
    pos_error::PosError,
    shared::{attr, text, SharedError},
    KunyomiError, PinYinError,
};
use kanjidic_types::Reading;
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReadingError {
    #[error("(Reading) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Reading) qc_type not recognized: {0}")]
    UnrecognizedType(PosError),
    #[error("(Reading) Pin yin: {0}")]
    PinYin(#[from] PinYinError),
    #[error("(Reading) Kunyomi: {0}")]
    Kunyomi(#[from] KunyomiError),
}

pub fn from(node: Node) -> Result<Reading, ReadingError> {
    let r_type = attr(node, "r_type")?;
    match r_type {
        "pinyin" => Ok(Reading::PinYin(pin_yin::from(node)?)),
        "korean_r" => Ok(Reading::KoreanRomanized {
            text: text(node)?.into(),
        }),
        "korean_h" => Ok(Reading::KoreanHangul {
            text: text(node)?.into(),
        }),
        "vietnam" => Ok(Reading::Vietnam {
            text: text(node)?.into(),
        }),
        "ja_on" => Ok(Reading::Onyomi {
            text: text(node)?.into(),
        }),
        "ja_kun" => Ok(Reading::Kunyomi(kunyomi::from(node)?)),
        _ => Err(ReadingError::UnrecognizedType(PosError::from(node))),
    }
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{PinYin, Reading, Tone};

    #[test]
    fn reading() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("reading"))
            .unwrap();
        let reading = from(node);
        assert_eq!(
            reading,
            Ok(Reading::PinYin(PinYin {
                romanization: "ya".into(),
                tone: Tone::Falling,
            }))
        )
    }
}
