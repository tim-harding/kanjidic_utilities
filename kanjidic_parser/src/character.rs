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
                    Codepoint::Unicode { code: 20124 },
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
                    Reference::NelsonClassic { index: 43 },
                    Reference::NelsonNew { index: 81 },
                    Reference::Njecd { index: 3540 },
                    Reference::Kkd { index: 4354 },
                    Reference::Kkld { index: 2204 },
                    Reference::Kkld2ed { index: 2966 },
                    Reference::Heisig { index: 1809 },
                    Reference::Heisig6 { index: 1950 },
                    Reference::Gakken { index: 1331 },
                    Reference::OneillNames(Oneill {
                        index: 525,
                        suffix: OneillSuffix::None,
                    }),
                    Reference::OneillKk { index: 1788 },
                    Reference::Moro(Moro {
                        volume: Some(1),
                        page: Some(525),
                        index: MoroIndex {
                            index: 272,
                            suffix: MoroSuffix::None,
                        },
                    }),
                    Reference::Henshall { index: 997 },
                    Reference::ShKk { index: 1616 },
                    Reference::ShKk2 { index: 1724 },
                    Reference::Jfcards { index: 1032 },
                    Reference::TuttleCards { index: 1092 },
                    Reference::KanjiInContext { index: 1818 },
                    Reference::KodanshaCompact { index: 35 },
                    Reference::Maniette { index: 1827 },
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
                        Reading::KoreanRomanized { text: "a".into() },
                        Reading::KoreanHangul { text: "아".into() },
                        Reading::Vietnam { text: "A".into() },
                        Reading::Vietnam { text: "Á".into() },
                        Reading::Onyomi { text: "ア".into() },
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
