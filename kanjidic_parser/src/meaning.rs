use crate::{
    reading,
    shared::{children, SharedError},
    translation::{self, TranslationError},
    ReadingError,
};
use kanjidic_types::Meaning;
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MeaningError {
    #[error("(Meaning) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Meaning) Reading: {0}")]
    Reading(#[from] ReadingError),
    #[error("(Meaning) Translation: {0}")]
    Translation(#[from] TranslationError),
}

pub fn from(rmgroup: Node) -> Result<Meaning, MeaningError> {
    let readings = children(rmgroup, "reading", reading::from)?;
    let translations = children(rmgroup, "meaning", translation::from)?;
    Ok(Meaning {
        readings,
        translations,
    })
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{
        Kunyomi, KunyomiKind, LanguageCode, Meaning, PinYin, Reading, Tone, Translation,
    };

    #[test]
    fn meaning() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("reading_meaning"))
            .unwrap();
        let meaning = from(node);
        assert_eq!(
            meaning,
            Ok(Meaning {
                readings: vec![
                    Reading::PinYin(PinYin {
                        romanization: "ya".into(),
                        tone: Tone::Falling,
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
                        language: LanguageCode::Eng,
                    },
                    Translation {
                        text: "rank next".into(),
                        language: LanguageCode::Eng,
                    },
                    Translation {
                        text: "come after".into(),
                        language: LanguageCode::Eng,
                    },
                    Translation {
                        text: "-ous".into(),
                        language: LanguageCode::Eng,
                    },
                    Translation {
                        text: "Asie".into(),
                        language: LanguageCode::Fra,
                    },
                    Translation {
                        text: "suivant".into(),
                        language: LanguageCode::Fra,
                    },
                    Translation {
                        text: "sub-".into(),
                        language: LanguageCode::Fra,
                    },
                    Translation {
                        text: "sous-".into(),
                        language: LanguageCode::Fra,
                    },
                    Translation {
                        text: "pref. para indicar".into(),
                        language: LanguageCode::Spa,
                    },
                    Translation {
                        text: "venir después de".into(),
                        language: LanguageCode::Spa,
                    },
                    Translation {
                        text: "Asia".into(),
                        language: LanguageCode::Spa,
                    },
                    Translation {
                        text: "Ásia".into(),
                        language: LanguageCode::Por,
                    },
                    Translation {
                        text: "próxima".into(),
                        language: LanguageCode::Por,
                    },
                    Translation {
                        text: "o que vem depois".into(),
                        language: LanguageCode::Por,
                    },
                    Translation {
                        text: "-ous".into(),
                        language: LanguageCode::Por,
                    },
                ],
            })
        )
    }
}
