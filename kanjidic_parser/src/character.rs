use crate::{
    codepoint, grade, query_code, radical, reading, reference,
    shared::{children, text, text_uint, SharedError},
    translation, variant, CodepointError, GradeError, PosError, QueryCodeError,
    RadicalError, ReadingError, ReferenceError, StrokeCountBuilder, StrokeCountError,
    TranslationError, VariantError,
};
use kanjidic_types::{
    Character, Codepoint, Grade, QueryCode, Radical, Reading, Reference, StrokeCount, Translations,
    Variant,
};
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
    #[error("(Character) Translation: {0}")]
    Translation(#[from] TranslationError),
    #[error("(Character) Reading: {0}")]
    Reading(#[from] ReadingError),
    #[error("(Character) Query code: {0}")]
    QueryCode(#[from] QueryCodeError),
    #[error("(Character) Dictionary reference: {0}")]
    DictionaryReference(#[from] ReferenceError),
    #[error("(Character) Nanori node missing text: {0}")]
    NanoriText(PosError),
    #[error("(Character) Expected a single char")]
    NonCharString,
    #[error("(Character) Character did not have non-optional fields")]
    IncompleteCharacter,
}

struct CharacterBuilder {
    /// The character itself.
    pub literal: Option<char>,
    /// Alternate encodings for the character.
    pub codepoints: Option<Vec<Codepoint>>,
    /// Alternate classifications for the character by radical.
    pub radicals: Option<Vec<Radical>>,
    /// The kanji grade level.
    pub grade: Option<Grade>,
    /// The stroke count of the character.
    pub stroke_counts: Option<StrokeCount>,
    /// Cross-references to other characters or alternative indexings.
    pub variants: Option<Vec<Variant>>,
    /// A ranking of how often the character appears in newspapers.
    pub frequency: Option<u16>,
    /// The kanji's name as a radical if it is one.
    pub radical_names: Option<Vec<String>>,
    /// Old JLPT level of the kanji. Based on pre-2010 test levels
    /// that go up to four, not five.
    pub jlpt: Option<u8>,
    /// Indexes into dictionaries and other instructional books
    pub references: Option<Vec<Reference>>,
    /// Codes used to identify the kanji
    pub query_codes: Option<Vec<QueryCode>>,
    /// Different ways the kanji can be read.
    pub readings: Option<Vec<Reading>>,
    /// Translations of the kanji into different languages.
    pub translations: Option<Translations>,
    /// Japanese readings associated with names.
    pub nanori: Option<Vec<String>>,
    /// The constituent radicals in the kanji
    pub decomposition: Option<Vec<char>>,
}

impl CharacterBuilder {
    pub fn new() -> Self {
        Self {
            literal: None,
            codepoints: None,
            radicals: None,
            grade: None,
            stroke_counts: None,
            variants: None,
            frequency: None,
            radical_names: None,
            jlpt: None,
            references: None,
            query_codes: None,
            readings: None,
            translations: None,
            nanori: None,
            decomposition: None,
        }
    }

    fn build(self) -> Result<Character, CharacterError> {
        let literal = self.literal.ok_or(CharacterError::IncompleteCharacter)?;
        let codepoints = self.codepoints.unwrap_or(vec![]);
        let radicals = self.radicals.unwrap_or(vec![]);
        let grade = self.grade;
        let stroke_counts = self
            .stroke_counts
            .ok_or(CharacterError::IncompleteCharacter)?;
        let variants = self.variants.unwrap_or(vec![]);
        let frequency = self.frequency;
        let radical_names = self.radical_names.unwrap_or(vec![]);
        let jlpt = self.jlpt;
        let references = self.references.unwrap_or(vec![]);
        let query_codes = self.query_codes.unwrap_or(vec![]);
        let readings = self.readings.unwrap_or(vec![]);
        let translations = self.translations.unwrap_or(Translations::default());
        let nanori = self.nanori.unwrap_or(vec![]);
        let decomposition = self.decomposition.unwrap_or(vec![]);

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
            readings,
            translations,
            nanori,
            decomposition,
        })
    }
}

pub fn string_to_char(s: &str) -> Result<char, CharacterError> {
    let mut chars = s.chars();
    let radical = chars.next().ok_or(CharacterError::NonCharString);
    match chars.next() {
        Some(_) => Err(CharacterError::NonCharString),
        None => radical,
    }
}

pub fn from(character_node: Node) -> Result<Character, CharacterError> {
    let mut builder = CharacterBuilder::new();
    for child in character_node.children() {
        match child.tag_name().name() {
            "literal" => {
                let literal = string_to_char(text(&child)?)?;
                builder.literal = Some(literal);
                builder.decomposition = Some(decomposition(literal));
            }
            "codepoint" => {
                builder.codepoints = Some(children(&child, "cp_value", codepoint::from)?);
            }
            "radical" => {
                builder.radicals = Some(children(&child, "rad_value", radical::from)?);
            }
            "misc" => {
                unpack_misc(&child, &mut builder)?;
            }
            "dic_number" => {
                builder.references = Some(children(&child, "dic_ref", reference::from)?);
            }
            "query_code" => {
                builder.query_codes = Some(children(&child, "q_code", query_code::from)?);
            }
            "reading_meaning" => {
                unpack_reading_meaning(&child, &mut builder)?;
            }
            _ => {}
        }
    }
    builder.build()
}

fn unpack_reading_meaning(
    reading_meaning: &Node,
    builder: &mut CharacterBuilder,
) -> Result<(), CharacterError> {
    let mut nanori = vec![];
    for child in reading_meaning.children() {
        match child.tag_name().name() {
            "rmgroup" => {
                unpack_rmgroup(&child, builder)?;
            }
            "nanori" => {
                nanori.push(
                    text(&child)
                        .map(|s: &str| s.to_owned())
                        .map_err(|_| CharacterError::NanoriText(PosError::from(reading_meaning)))?,
                );
            }
            _ => {}
        }
    }
    builder.nanori = Some(nanori);
    Ok(())
}

fn unpack_rmgroup(rmgroup: &Node, builder: &mut CharacterBuilder) -> Result<(), CharacterError> {
    let mut readings = vec![];
    let mut translations = Translations::default();
    for child in rmgroup.children() {
        match child.tag_name().name() {
            "reading" => {
                readings.push(reading::from(child)?);
            }
            "meaning" => {
                translation::add_meaning(&mut translations, &child)?;
            }
            _ => {}
        }
    }
    builder.readings = Some(readings);
    builder.translations = Some(translations);
    Ok(())
}

fn unpack_misc(misc: &Node, builder: &mut CharacterBuilder) -> Result<(), CharacterError> {
    let mut variants = vec![];
    let mut radical_names = vec![];
    let mut stroke_counts = StrokeCountBuilder::new();
    for child in misc.children() {
        match child.tag_name().name() {
            "grade" => {
                builder.grade = Some(grade::from(child)?);
            }
            "variant" => variants.push(variant::from(child)?),
            "freq" => {
                builder.frequency = Some(text_uint::<u16>(&child)?);
            }
            "rad_name" => {
                radical_names.push(text(&child)?.to_owned());
            }
            "jlpt" => {
                builder.jlpt = Some(text_uint::<u8>(&child)?);
            }
            "stroke_count" => {
                stroke_counts.add_from_node(&child)?;
            }
            _ => {}
        }
    }
    builder.variants = Some(variants);
    builder.radical_names = Some(radical_names);
    builder.stroke_counts = Some(stroke_counts.build()?);
    Ok(())
}

fn decomposition(literal: char) -> Vec<char> {
    for decomposition in kradical_static::DECOMPOSITIONS {
        if decomposition.kanji == literal {
            let out: Vec<char> = decomposition.radicals.iter().map(|&c| c).collect();
            return out;
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};

    use kanjidic_types::{
        Character, Codepoint, DeRoo, ExtremeBottom, ExtremeTop, FourCorner, Grade, KangXi, Kunyomi,
        KunyomiKind, Kuten, Moro, MoroSuffix, Oneill, OneillSuffix, PinYin, QueryCode, Radical,
        RadicalKind, Reading, Reference, ShDesc, Skip, SkipSolid, SolidSubpattern, Stroke,
        StrokeCount, Tone, Variant,
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
                decomposition: Some(vec!["｜".into(), "一".into(), "口".into()]),
                codepoints: vec![
                    Codepoint::Unicode(20124),
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
                        index: 272,
                        suffix: MoroSuffix::None,
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
                translations: HashMap::from_iter([
                    (
                        "en".to_owned(),
                        vec![
                            "Asia".to_owned(),
                            "rank next".to_owned(),
                            "come after".to_owned(),
                            "-ous".to_owned(),
                        ]
                    ),
                    (
                        "fr".to_owned(),
                        vec![
                            "Asie".to_owned(),
                            "suivant".to_owned(),
                            "sub-".to_owned(),
                            "sous-".to_owned(),
                        ]
                    ),
                    (
                        "pt".to_owned(),
                        vec![
                            "Ásia".to_owned(),
                            "próxima".to_owned(),
                            "o que vem depois".to_owned(),
                            "-ous".to_owned(),
                        ]
                    ),
                    (
                        "es".to_owned(),
                        vec![
                            "pref. para indicar".to_owned(),
                            "venir después de".to_owned(),
                            "Asia".to_owned(),
                        ]
                    )
                ])
            })
        )
    }
}
