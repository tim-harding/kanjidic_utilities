use std::convert::TryFrom;

use crate::{
    reading::{Reading, ReadingError},
    translation::{Translation, TranslationError},
};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MeaningError {
    #[error("Nanori node is missing text content")]
    NanoriText,
    #[error("No rmgroup tag")]
    MissingGroup,
    #[error("Error while parsing reading")]
    Reading(#[from] ReadingError),
    #[error("Error while parsing translation")]
    Translation(#[from] TranslationError),
}

/// Information about a particular meaning of a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Meaning<'a> {
    /// Different ways the kanji can be read.
    pub readings: Vec<Reading<'a>>,
    /// Translations of the kanji into different languages.
    pub translations: Vec<Translation<'a>>,
    /// Japanese readings associated with names.
    pub nanori: Vec<&'a str>,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Meaning<'a> {
    type Error = MeaningError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let nanori: Option<Vec<&str>> = node
            .children()
            .filter(|child| child.has_tag_name("nanori"))
            .map(|node| node.text())
            .collect();
        let nanori = nanori.ok_or(MeaningError::NanoriText)?;
        let rmgroup = node
            .children()
            .find(|child| child.has_tag_name("rmgroup"))
            .ok_or(MeaningError::MissingGroup)?;
        let readings: Result<Vec<Reading>, ReadingError> = rmgroup
            .children()
            .filter(|child| child.has_tag_name("reading"))
            .map(|node| Reading::try_from(node))
            .collect();
        let readings = readings?;
        let translations: Result<Vec<Translation>, TranslationError> = rmgroup
            .children()
            .filter(|child| child.has_tag_name("meaning"))
            .map(|node| Translation::try_from(node))
            .collect();
        let translations = translations?;
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
                nanori: vec!["や", "つぎ", "つぐ",],
                readings: vec![
                    Reading::PinYin(PinYin {
                        romanization: "ya",
                        tone: crate::pin_yin::Tone::Falling,
                    }),
                    Reading::KoreanRomanized("a"),
                    Reading::KoreanHangul("아"),
                    Reading::Vietnam("A"),
                    Reading::Vietnam("Á"),
                    Reading::Onyomi("ア"),
                    Reading::Kunyomi(Kunyomi {
                        kind: KunyomiKind::Normal,
                        okurigana: vec!["つ", "ぐ",]
                    })
                ],
                translations: vec![
                    Translation {
                        text: "Asia",
                        language: Language::Eng,
                    },
                    Translation {
                        text: "rank next",
                        language: Language::Eng,
                    },
                    Translation {
                        text: "come after",
                        language: Language::Eng,
                    },
                    Translation {
                        text: "-ous",
                        language: Language::Eng,
                    },
                    Translation {
                        text: "Asie",
                        language: Language::Fra,
                    },
                    Translation {
                        text: "suivant",
                        language: Language::Fra,
                    },
                    Translation {
                        text: "sub-",
                        language: Language::Fra,
                    },
                    Translation {
                        text: "sous-",
                        language: Language::Fra,
                    },
                    Translation {
                        text: "pref. para indicar",
                        language: Language::Spa,
                    },
                    Translation {
                        text: "venir después de",
                        language: Language::Spa,
                    },
                    Translation {
                        text: "Asia",
                        language: Language::Spa,
                    },
                    Translation {
                        text: "Ásia",
                        language: Language::Por,
                    },
                    Translation {
                        text: "próxima",
                        language: Language::Por,
                    },
                    Translation {
                        text: "o que vem depois",
                        language: Language::Por,
                    },
                    Translation {
                        text: "-ous",
                        language: Language::Por,
                    },
                ],
            })
        )
    }
}
