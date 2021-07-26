use std::convert::TryFrom;

use crate::{
    codepoint::{Codepoint, CodepointError},
    dictionary_reference::{Reference, ReferenceError},
    grade::{Grade, GradeError},
    meaning::{Meaning, MeaningError},
    query_code::{QueryCode, QueryCodeError},
    radical::{Radical, RadicalError},
    shared::{child, children, text, text_uint, SharedError},
    stroke_count::{StrokeCount, StrokeCountError},
    variant::{Variant, VariantError},
};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CharacterError {
    #[error("(Character) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Character) Codepoint: {0}")]
    Codepoint(#[from] CodepointError),
    #[error("(Character) Radical: {0}")]
    Radical(#[from] RadicalError),
    #[error("(Character) Grade: {0}")]
    Grade(#[from] GradeError),
    #[error("(Character) Stroke count: {0}")]
    StrokeCount(#[from] StrokeCountError),
    #[error("(Character) Variant: {0}")]
    Variant(#[from] VariantError),
    #[error("(Character) Meaning: {0}")]
    Meaning(#[from] MeaningError),
    #[error("(Character) Query code: {0}")]
    QueryCode(#[from] QueryCodeError),
    #[error("(Character) Dictionary reference: {0}")]
    DictionaryReference(#[from] ReferenceError),
}

/// Information about a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Character {
    /// The character itself.
    pub literal: String,
    /// Alternate encodings for the character.
    pub codepoints: Vec<Codepoint>,
    /// Alternate classifications for the character by radical.
    pub radicals: Vec<Radical>,
    /// The kanji grade level.
    pub grade: Option<Grade>,
    /// The stroke count of the character.
    pub stroke_counts: StrokeCount,
    /// Cross-references to other characters or alternative indexings.
    pub variants: Vec<Variant>,
    /// A ranking of how often the character appears in newspapers.
    pub frequency: Option<u16>,
    /// The kanji's name as a radical if it is one.
    pub radical_names: Vec<String>,
    /// Old JLPT level of the kanji. Based on pre-2010 test levels
    /// that go up to four, not five.
    pub jlpt: Option<u8>,
    /// Indexes into dictionaries and other instructional books
    pub references: Vec<Reference>,
    /// Codes used to identify the kanji
    pub query_codes: Vec<QueryCode>,
    /// Different meanings of the kanji.
    pub meanings: Vec<Meaning>,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Character {
    type Error = CharacterError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let literal = text(child(node, "literal")?)?.into();
        let codepoints = children(child(node, "codepoint")?, "cp_value", Codepoint::try_from)?;
        let radicals = children(child(node, "radical")?, "rad_value", Radical::try_from)?;
        let misc = child(node, "misc")?;
        let grade = coalesce(child(misc, "grade").ok().map(Grade::try_from))?;
        let stroke_counts = StrokeCount::try_from(misc)?;
        let variants = children(misc, "variant", Variant::try_from)?;
        let frequency = coalesce(child(misc, "freq").ok().map(text_uint::<u16>))?;
        let radical_names =
            children::<_, SharedError, _>(misc, "rad_name", |child| Ok(text(child)?.to_owned()))?;
        let jlpt = coalesce(child(misc, "jlpt").ok().map(text_uint::<u8>))?;
        let references = match child(node, "dic_number") {
            Ok(dic_number) => Ok(children(dic_number, "dic_ref", Reference::try_from)?),
            Err(SharedError::MissingChild(_, _)) => Ok(vec![]),
            Err(other) => Err(other),
        }?;
        let query_codes = children(child(node, "query_code")?, "q_code", QueryCode::try_from)?;
        let meanings = children(node, "reading_meaning", Meaning::try_from)?;
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

fn coalesce<T, E: std::error::Error>(opt: Option<Result<T, E>>) -> Result<Option<T>, E> {
    Ok(match opt {
        Some(v) => Some(v?),
        None => None,
    })
}

#[cfg(test)]
mod tests {
    use isolang::Language;

    use super::*;
    use crate::{
        de_roo::{DeRoo, ExtremeBottom, ExtremeTop},
        four_corner::{FourCorner, Stroke},
        kangxi::KangXi,
        kunyomi::{Kunyomi, KunyomiKind},
        kuten::Kuten,
        moro::{Moro, MoroIndex, MoroSuffix},
        oneill::{Oneill, OneillSuffix},
        pin_yin::PinYin,
        reading::Reading,
        skip::{Skip, SkipSolid, SolidSubpattern},
        spahn_hadamitzky::ShDesc,
        test_shared::DOC,
        translation::Translation,
    };

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
                literal: "亜".into(),
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
                stroke_counts: StrokeCount {
                    accepted: 7,
                    miscounts: vec![]
                },
                variants: vec![Variant::Jis208(Kuten {
                    plane: 1,
                    ku: 48,
                    ten: 19,
                })],
                frequency: Some(1509),
                jlpt: Some(1),
                references: vec![
                    Reference::NelsonClassic(43),
                    Reference::NelsonNew(81),
                    Reference::Njecd(3540),
                    Reference::Kkd(4354),
                    Reference::Kkld(2204),
                    Reference::Kkld2ed(2966),
                    Reference::Heisig(1809),
                    Reference::Heisig6(1950),
                    Reference::Gakken(1331),
                    Reference::OneillNames(Oneill {
                        number: 525,
                        suffix: OneillSuffix::None,
                    }),
                    Reference::OneillKk(1788),
                    Reference::Moro(Moro {
                        volume: Some(1),
                        page: Some(525),
                        index: MoroIndex {
                            number: 272,
                            suffix: MoroSuffix::None,
                        },
                    }),
                    Reference::Henshall(997),
                    Reference::ShKk(1616),
                    Reference::ShKk2(1724),
                    Reference::Jfcards(1032),
                    Reference::TuttleCards(1092),
                    Reference::KanjiInContext(1818),
                    Reference::KodanshaCompact(35),
                    Reference::Maniette(1827),
                ],
                query_codes: vec![
                    QueryCode::Skip(Skip::Solid(SkipSolid {
                        total_stroke_count: 7,
                        solid_subpattern: SolidSubpattern::TopLine,
                    })),
                    QueryCode::SpahnHadamitzky(ShDesc {
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
                }]
            })
        )
    }
}
