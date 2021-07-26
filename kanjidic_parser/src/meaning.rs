use std::convert::TryFrom;

use crate::{
    pos_error::PosError,
    reading::{Reading, ReadingError},
    shared::{child, children, text, SharedError},
    translation::{Translation, TranslationError},
};
use roxmltree::Node;
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MeaningError {
    #[error("(Meaning) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Meaning) Nanori node missing text: {0}")]
    NanoriText(PosError),
    #[error("(Meaning) Reading: {0}")]
    Reading(#[from] ReadingError),
    #[error("(Meaning) Translation: {0}")]
    Translation(#[from] TranslationError),
}

/// Information about a particular meaning of a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Meaning {
    /// Different ways the kanji can be read.
    pub readings: Vec<Reading>,
    /// Translations of the kanji into different languages.
    pub translations: Vec<Translation>,
    /// Japanese readings associated with names.
    pub nanori: Vec<String>,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Meaning {
    type Error = MeaningError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let nanori = children(node, "nanori", |child| {
            text(child).map(|s: &str| s.to_string()).map_err(|_| MeaningError::NanoriText(PosError::from(node)))
        })?;
        let rmgroup = child(node, "rmgroup")?;
        let readings = children(rmgroup, "reading", |child| Reading::try_from(child))?;
        let translations = children(rmgroup, "meaning", |child| Translation::try_from(child))?;
        Ok(Meaning {
            readings,
            translations,
            nanori,
        })
    }
}

#[cfg(test)]
mod tests {
    use isolang::Language;

    use super::*;
    use crate::{
        kunyomi::{Kunyomi, KunyomiKind},
        pin_yin::PinYin,
        test_shared::DOC,
    };

    #[test]
    fn meaning() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("reading_meaning"))
            .unwrap();
        let meaning = Meaning::try_from(node);
        assert_eq!(
            meaning,
            Ok(Meaning {
                nanori: vec!["や".into(), "つぎ".into(), "つぐ".into(),],
                readings: vec![
                    Reading::PinYin(PinYin {
                        romanization: "ya".into(),
                        tone: crate::pin_yin::Tone::Falling,
                    }),
                    Reading::KoreanRomanized("a".into()),
                    Reading::KoreanHangul("아".into()),
                    Reading::Vietnam("A".into()),
                    Reading::Vietnam("Á".into()),
                    Reading::Onyomi("ア".into()),
                    Reading::Kunyomi(Kunyomi {
                        kind: KunyomiKind::Normal,
                        okurigana: vec!["つ".into(), "ぐ".into(),]
                    })
                ],
                translations: vec![
                    Translation {
                        text: "Asia".into(),
                        language: Language::Eng,
                    },
                    Translation {
                        text: "rank next".into(),
                        language: Language::Eng,
                    },
                    Translation {
                        text: "come after".into(),
                        language: Language::Eng,
                    },
                    Translation {
                        text: "-ous".into(),
                        language: Language::Eng,
                    },
                    Translation {
                        text: "Asie".into(),
                        language: Language::Fra,
                    },
                    Translation {
                        text: "suivant".into(),
                        language: Language::Fra,
                    },
                    Translation {
                        text: "sub-".into(),
                        language: Language::Fra,
                    },
                    Translation {
                        text: "sous-".into(),
                        language: Language::Fra,
                    },
                    Translation {
                        text: "pref. para indicar".into(),
                        language: Language::Spa,
                    },
                    Translation {
                        text: "venir después de".into(),
                        language: Language::Spa,
                    },
                    Translation {
                        text: "Asia".into(),
                        language: Language::Spa,
                    },
                    Translation {
                        text: "Ásia".into(),
                        language: Language::Por,
                    },
                    Translation {
                        text: "próxima".into(),
                        language: Language::Por,
                    },
                    Translation {
                        text: "o que vem depois".into(),
                        language: Language::Por,
                    },
                    Translation {
                        text: "-ous".into(),
                        language: Language::Por,
                    },
                ],
            })
        )
    }
}
