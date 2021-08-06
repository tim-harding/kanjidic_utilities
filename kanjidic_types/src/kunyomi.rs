use serde::{Deserialize, Serialize};

/// A kunyomi kanji reading.
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Kunyomi {
    /// The okurigana
    pub okurigana: Vec<String>,
    /// Whether the reading is as a prefix or suffix.
    pub kind: KunyomiKind,
}

/// The kind of kunyomi reading.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum KunyomiKind {
    /// A normal reading
    Normal,
    /// A prefix
    Prefix,
    /// A suffix
    Suffix,
}
