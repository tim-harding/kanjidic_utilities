/// A kunyomi kanji reading.
pub struct Kunyomi<'a> {
    /// The okurigana
    pub okurigana: Vec<&'a str>,

    /// Whether the reading is as a prefix or suffix.
    pub kind: KunyomiKind,
}

/// The kind of kunyomi reading.
pub enum KunyomiKind {
    /// A normal reading
    Normal,

    /// A prefix
    Prefix,

    /// A suffix
    Suffix,
}
