use std::convert::TryFrom;

use crate::{
    kunyomi::{Kunyomi, KunyomiError},
    pin_yin::{PinYin, PinYinError},
    pos_error::PosError,
    shared::{attr, text, SharedError},
};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReadingError {
    #[error("Error from shared utilities")]
    Shared(#[from] SharedError),
    #[error("qc_type not recognized")]
    UnrecognizedType(PosError),
    #[error("Error while parsing pin yin reading")]
    PinYin(#[from] PinYinError),
    #[error("Error while parsing kunyomi reading")]
    Kunyomi(#[from] KunyomiError),
}

/// A particular reading or pronunciation of a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Reading<'a> {
    /// The modern romanization of the Chinese reading.
    PinYin(PinYin<'a>),
    /// The romanized form of the Korean reading.
    KoreanRomanized(&'a str),
    /// The Korean reading of the kanji in Hangul.
    KoreanHangul(&'a str),
    /// The Vietnamese reading supplied by Minh Chau Pham.
    Vietnam(&'a str),
    /// The onyomi reading of the kanji in katakana.
    Onyomi(&'a str),
    /// The kunyomi reading of the kanji in hiragana or katakana.
    Kunyomi(Kunyomi<'a>),
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Reading<'a> {
    type Error = ReadingError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let r_type = attr(node, "r_type")?;
        match r_type {
            "pinyin" => Ok(Reading::PinYin(PinYin::try_from(node)?)),
            "korean_r" => Ok(Reading::KoreanRomanized(text(node)?)),
            "korean_h" => Ok(Reading::KoreanHangul(text(node)?)),
            "vietnam" => Ok(Reading::Vietnam(text(node)?)),
            "ja_on" => Ok(Reading::Onyomi(text(node)?)),
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
                romanization: "ya",
                tone: Tone::Falling,
            }))
        )
    }
}
