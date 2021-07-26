use std::convert::TryFrom;

use crate::{
    kunyomi::{Kunyomi, KunyomiError},
    pin_yin::{PinYin, PinYinError},
    pos_error::PosError,
    shared::{attr, text, SharedError},
};
use roxmltree::Node;
use thiserror::Error;
use serde::{Serialize, Deserialize};

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

/// A particular reading or pronunciation of a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Reading {
    /// The modern romanization of the Chinese reading.
    PinYin(PinYin),
    /// The romanized form of the Korean reading.
    KoreanRomanized(String),
    /// The Korean reading of the kanji in Hangul.
    KoreanHangul(String),
    /// The Vietnamese reading supplied by Minh Chau Pham.
    Vietnam(String),
    /// The onyomi reading of the kanji in katakana.
    Onyomi(String),
    /// The kunyomi reading of the kanji in hiragana or katakana.
    Kunyomi(Kunyomi),
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Reading {
    type Error = ReadingError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let r_type = attr(node, "r_type")?;
        match r_type {
            "pinyin" => Ok(Reading::PinYin(PinYin::try_from(node)?)),
            "korean_r" => Ok(Reading::KoreanRomanized(text(node)?.into())),
            "korean_h" => Ok(Reading::KoreanHangul(text(node)?.into())),
            "vietnam" => Ok(Reading::Vietnam(text(node)?.into())),
            "ja_on" => Ok(Reading::Onyomi(text(node)?.into())),
            "ja_kun" => Ok(Reading::Kunyomi(Kunyomi::try_from(node)?)),
            _ => Err(ReadingError::UnrecognizedType(PosError::from(node))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Reading;
    use crate::{
        pin_yin::{PinYin, Tone},
        test_shared::DOC,
    };
    use std::convert::TryFrom;

    #[test]
    fn reading() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("reading"))
            .unwrap();
        let reading = Reading::try_from(node);
        assert_eq!(
            reading,
            Ok(Reading::PinYin(PinYin {
                romanization: "ya".into(),
                tone: Tone::Falling,
            }))
        )
    }
}
