pub struct Kunyomi<'a> {
    pub okurigana: Vec<&'a str>,
    pub kind: KunyomiKind,
}

pub enum KunyomiKind {
    Normal,
    Prefix,
    Suffix,
}
