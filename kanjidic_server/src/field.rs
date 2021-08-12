use serde::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Hash, FromFormField)]
pub enum Field {
    All,
    Codepoints,
    Radicals,
    Grade,
    StrokeCounts,
    Variants,
    Frequency,
    RadicalNames,
    Jlpt,
    References,
    QueryCodes,
    Readings,
    Translations,
    Nanori,
    Decomposition,
}
