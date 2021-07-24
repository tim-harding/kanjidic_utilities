use std::convert::TryFrom;

use crate::{
    codepoint::{Codepoint, CodepointError},
    dictionary_reference::{DictionaryReference, DictionaryReferenceError},
    grade::{Grade, GradeError},
    meaning::{Meaning, MeaningError},
    query_code::{QueryCode, QueryCodeError},
    radical::{Radical, RadicalError},
    stroke_count::{StrokeCount, StrokeCountError},
    variant::{Variant, VariantError},
};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CharacterError {
    #[error("Could not find kanji literal node")]
    MissingLiteral,
    #[error("Literal node was empty")]
    EmptyLiteral,
    #[error("Could not find codepoint node")]
    MissingCodepoint,
    #[error("Could not find radical node")]
    MissingRadical,
    #[error("Could not find misc node")]
    MissingMisc,
    #[error("Freqency node contains no text")]
    FrequencyText,
    #[error("Could not parse the kanji frequency")]
    FrequencyParse,
    #[error("JLPT node contains no text")]
    JlptText,
    #[error("Could not parse the JLPT level")]
    JlptParse,
    #[error("Radical name node did not contain text")]
    RadicalNameText,
    #[error("Could not find the dic_number node")]
    MissingDictionaryReferences,
    #[error("Could not find the query_code node")]
    MissingQueryCodes,
    #[error("Error parsing a codepoint")]
    Codepoint(#[from] CodepointError),
    #[error("Error parsing a radical")]
    Radical(#[from] RadicalError),
    #[error("Error parsing a grade")]
    Grade(#[from] GradeError),
    #[error("Error parsing a stroke count")]
    StrokeCount(#[from] StrokeCountError),
    #[error("Error parsing a variant")]
    Variant(#[from] VariantError),
    #[error("Error parsing a meaning")]
    Meaning(#[from] MeaningError),
    #[error("Error parsing a query code")]
    QueryCode(#[from] QueryCodeError),
    #[error("Error parsing a dictionary reference")]
    DictionaryReference(#[from] DictionaryReferenceError),
}

/// Information about a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Character<'a> {
    /// The character itself.
    pub literal: &'a str,
    /// Alternate encodings for the character.
    pub codepoints: Vec<Codepoint>,
    /// Alternate classifications for the character by radical.
    pub radicals: Vec<Radical>,
    /// The kanji grade level.
    pub grade: Option<Grade>,
    /// The stroke count of the character.
    pub stroke_counts: Vec<StrokeCount>,
    /// Cross-references to other characters or alternative indexings.
    pub variants: Vec<Variant>,
    /// A ranking of how often the character appears in newspapers.
    pub frequency: Option<u16>,
    /// The kanji's name as a radical if it is one.
    pub radical_names: Vec<&'a str>,
    /// Old JLPT level of the kanji. Based on pre-2010 test levels
    /// that go up to four, not five.
    pub jlpt: Option<u8>,
    /// Indexes into dictionaries and other instructional books
    pub references: Vec<DictionaryReference>,
    /// Codes used to identify the kanji
    pub query_codes: Vec<QueryCode>,
    /// Different meanings of the kanji.
    pub meanings: Vec<Meaning<'a>>,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Character<'a> {
    type Error = CharacterError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let literal = node
            .children()
            .find(|child| child.has_tag_name("literal"))
            .ok_or(CharacterError::MissingLiteral)?
            .text()
            .ok_or(CharacterError::EmptyLiteral)?;
        let codepoints: Result<Vec<Codepoint>, CodepointError> = node
            .children()
            .find(|child| child.has_tag_name("codepoint"))
            .ok_or(CharacterError::MissingCodepoint)?
            .children()
            .filter(|child| child.has_tag_name("cp_value"))
            .map(|node| Codepoint::try_from(node))
            .collect();
        let codepoints = codepoints?;
        let radicals: Result<Vec<Radical>, RadicalError> = node
            .children()
            .find(|child| child.has_tag_name("radical"))
            .ok_or(CharacterError::MissingRadical)?
            .children()
            .filter(|child| child.has_tag_name("rad_value"))
            .map(|node| Radical::try_from(node))
            .collect();
        let radicals = radicals?;
        let misc = node
            .children()
            .find(|child| child.has_tag_name("misc"))
            .ok_or(CharacterError::MissingMisc)?;
        let grade = misc
            .children()
            .find(|child| child.has_tag_name("grade"))
            .map(|node| Grade::try_from(node));
        let grade = match grade {
            Some(grade) => Some(grade?),
            None => None,
        };
        let stroke_counts: Result<Vec<StrokeCount>, StrokeCountError> = misc
            .children()
            .filter(|child| child.has_tag_name("stroke_count"))
            .map(|node| StrokeCount::try_from(node))
            .collect();
        let stroke_counts = stroke_counts?;
        let variants: Result<Vec<Variant>, VariantError> = misc
            .children()
            .filter(|child| child.has_tag_name("variant"))
            .map(|node| Variant::try_from(node))
            .collect();
        let variants = variants?;
        let frequency = misc
            .children()
            .find(|child| child.has_tag_name("freq"))
            .map(|node| {
                node.text()
                    .ok_or(CharacterError::FrequencyText)?
                    .parse::<u16>()
                    .map_err(|_| CharacterError::FrequencyParse)
            });
        let frequency = match frequency {
            Some(frequency) => Some(frequency?),
            None => None,
        };
        let radical_names: Result<Vec<&str>, CharacterError> = misc
            .children()
            .filter(|child| child.has_tag_name("rad_name"))
            .map(|node| node.text().ok_or(CharacterError::RadicalNameText))
            .collect();
        let radical_names = radical_names?;
        let jlpt = misc
            .children()
            .find(|child| child.has_tag_name("jlpt"))
            .map(|node| {
                node.text()
                    .ok_or(CharacterError::JlptText)?
                    .parse::<u8>()
                    .map_err(|_| CharacterError::JlptParse)
            });
        let jlpt = match jlpt {
            Some(jlpt) => Some(jlpt?),
            None => None,
        };
        let references: Result<Vec<DictionaryReference>, DictionaryReferenceError> = node
            .children()
            .find(|child| child.has_tag_name("dic_number"))
            .ok_or(CharacterError::MissingDictionaryReferences)?
            .children()
            .filter(|child| child.has_tag_name("dic_ref"))
            .map(|node| DictionaryReference::try_from(node))
            .collect();
        let references = references?;
        let query_codes: Result<Vec<QueryCode>, QueryCodeError> = node
            .children()
            .find(|child| child.has_tag_name("query_code"))
            .ok_or(CharacterError::MissingQueryCodes)?
            .children()
            .filter(|child| child.has_tag_name("q_code"))
            .map(|node| QueryCode::try_from(node))
            .collect();
        let query_codes = query_codes?;
        let meanings: Result<Vec<Meaning>, MeaningError> = node
            .children()
            .filter(|child| child.has_tag_name("reading_meaning"))
            .map(|node| Meaning::try_from(node))
            .collect();
        let meanings = meanings?;
        Ok(Character {
            literal,
            codepoints,
            radicals,
            grade,
            stroke_counts,
            variants,
            frequency,
            radical_names,
            jlpt,
            references,
            query_codes,
            meanings,
        })
    }
}

#[cfg(test)]
mod tests {
    use isolang::Language;

    use super::*;
    use crate::{de_roo::{DeRoo, ExtremeBottom, ExtremeTop}, four_corner::{FourCorner, Stroke}, kangxi::KangXi, kunyomi::{Kunyomi, KunyomiKind}, kuten::Kuten, moro::Moro, pin_yin::PinYin, reading::Reading, skip::{Skip, SkipSolid, SolidSubpattern}, spahn_hadamitzky::SpahnHadamitzkyDescriptor, test_shared::DOC, translation::Translation};

    #[test]
    fn character() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("character"))
            .unwrap();
        let character = Character::try_from(node);
        assert_eq!(
            character,
            Ok(Character {
                literal: "亜",
                codepoints: vec![
                    Codepoint::Unicode(20124),
                    Codepoint::Jis208(Kuten {
                        plane: 1,
                        ku: 16,
                        ten: 1,
                    })
                ],
                radicals: vec![
                    Radical::Classical(KangXi::Two),
                    Radical::Nelson(KangXi::One),
                ],
                grade: Some(Grade::Jouyou),
                stroke_counts: vec![StrokeCount {
                    accepted: 7,
                    miscounts: vec![]
                }],
                variants: vec![Variant::Jis208(Kuten {
                    plane: 1,
                    ku: 48,
                    ten: 19,
                })],
                frequency: Some(1509),
                jlpt: Some(1),
                references: vec![
                    DictionaryReference::NelsonClassic(43),
                    DictionaryReference::NelsonNew(81),
                    DictionaryReference::Njecd(3540),
                    DictionaryReference::Kkd(4354),
                    DictionaryReference::Kkld(2204),
                    DictionaryReference::Kkld2ed(2966),
                    DictionaryReference::Heisig(1809),
                    DictionaryReference::Heisig6(1950),
                    DictionaryReference::Gakken(1331),
                    DictionaryReference::OneillNames(525),
                    DictionaryReference::OneillKk(1788),
                    DictionaryReference::Moro(Moro {
                        volume: Some(1),
                        page: Some(525),
                        item: 272,
                    }),
                    DictionaryReference::Henshall(997),
                    DictionaryReference::ShKk(1616),
                    DictionaryReference::ShKk2(1724),
                    DictionaryReference::Jfcards(1032),
                    DictionaryReference::TuttleCards(1092),
                    DictionaryReference::KanjiInContext(1818),
                    DictionaryReference::KodanshaCompact(35),
                    DictionaryReference::Maniette(1827),
                ],
                query_codes: vec![
                    QueryCode::Skip(Skip::Solid(SkipSolid {
                        total_stroke_count: 7,
                        solid_subpattern: SolidSubpattern::TopLine,
                    })),
                    QueryCode::SpahnHadamitzky(SpahnHadamitzkyDescriptor {
                        radical_strokes: 0,
                        radical: 'a',
                        other_strokes: 7,
                        sequence: 14,
                    }),
                    QueryCode::FourCorner(FourCorner {
                        top_left: Stroke::LineHorizontal,
                        top_right: Stroke::Lid,
                        bottom_left: Stroke::LineHorizontal,
                        bottom_right: Stroke::Lid,
                        fifth_corner: Some(Stroke::Box),
                    }),
                    QueryCode::DeRoo(DeRoo {
                        top: ExtremeTop::Bald,
                        bottom: ExtremeBottom::StandingBottom,
                    }),
                ],
                radical_names: vec![],
                meanings: vec![Meaning {
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
                }]
            })
        )
    }
}
