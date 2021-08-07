use crate::{
    codepoint, grade, meaning, query_code, radical, reference,
    shared::{child, children, text, text_uint, SharedError},
    stroke_count, variant, CodepointError, GradeError, MeaningError, QueryCodeError, RadicalError,
    ReferenceError, StrokeCountError, VariantError,
};
use kanjidic_types::Character;
use roxmltree::Node;
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

pub fn from(node: Node) -> Result<Character, CharacterError> {
    let literal = text(child(node, "literal")?)?.into();
    let codepoints = children(child(node, "codepoint")?, "cp_value", codepoint::from)?;
    let radicals = children(child(node, "radical")?, "rad_value", radical::from)?;
    let misc = child(node, "misc")?;
    let grade = coalesce(child(misc, "grade").ok().map(grade::from))?;
    let stroke_counts = stroke_count::from(misc)?;
    let variants = children(misc, "variant", variant::from)?;
    let frequency = coalesce(child(misc, "freq").ok().map(text_uint::<u16>))?;
    let radical_names =
        children::<_, SharedError, _>(misc, "rad_name", |child| Ok(text(child)?.to_owned()))?;
    let jlpt = coalesce(child(misc, "jlpt").ok().map(text_uint::<u8>))?;
    let references = match child(node, "dic_number") {
        Ok(dic_number) => Ok(children(dic_number, "dic_ref", reference::from)?),
        Err(SharedError::MissingChild(_, _)) => Ok(vec![]),
        Err(other) => Err(other),
    }?;
    let query_codes = children(child(node, "query_code")?, "q_code", query_code::from)?;
    let meanings = children(node, "reading_meaning", meaning::from)?;
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

fn coalesce<T, E: std::error::Error>(opt: Option<Result<T, E>>) -> Result<Option<T>, E> {
    Ok(match opt {
        Some(v) => Some(v?),
        None => None,
    })
}

#[cfg(test)]
mod tests {
    use kanjidic_types::{
        Character, Codepoint, DeRoo, ExtremeBottom, ExtremeTop, FourCorner, Grade, KangXi, Kunyomi,
        KunyomiKind, Kuten, LanguageCode, Meaning, Moro, MoroIndex, MoroSuffix, Oneill,
        OneillSuffix, PinYin, QueryCode, Radical, RadicalKind, Reading, Reference, ShDesc, Skip,
        SkipSolid, SolidSubpattern, Stroke, StrokeCount, Tone, Translation, Variant,
    };

    use super::from;
    use crate::test_shared::DOC;

    #[test]
    fn character() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("character"))
            .unwrap();
        let character = from(node);
        assert_eq!(
            character,
            Ok(Character {
                literal: "亜".into(),
                codepoints: vec![
                    Codepoint::Unicode { codepoint: 20124 },
                    Codepoint::Jis208(Kuten {
                        plane: 1,
                        ku: 16,
                        ten: 1,
                    })
                ],
                radicals: vec![
                    Radical {
                        kind: RadicalKind::Classical,
                        radical: KangXi::Two,
                    },
                    Radical {
                        kind: RadicalKind::Nelson,
                        radical: KangXi::One,
                    },
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
                }]
            })
        )
    }
}
