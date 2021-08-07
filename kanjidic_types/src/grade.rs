use serde::{Deserialize, Serialize};

/// The grade level in which the kanji is learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum Grade {
    /// A Kyouiku kanji learned in grades 1-6.
    Kyouiku { year: u8 },
    /// A remaining Jouyou kanji to be learned in junior hi-school.
    Jouyou,
    /// A Jinmeiyou kanji for use in names that is approved
    /// for use in family name registers and other official documents.
    Jinmeiyou,
    /// A Jinmeiyou kanji that is a variant of a Jouyou kanji.
    JinmeiyouJouyouVariant,
}
