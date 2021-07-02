pub struct Character<'a> {
    pub literal: Literal<'a>,
    pub codepoints: Vec<Codepoint>,
    pub radicals: Vec<Radical>,
    pub grade: Grade,
    pub stroke_count: StrokeCount,
}

pub struct Literal<'a>(pub &'a str);

// http://unicode-iphone.blogspot.com/2010/05/kuten-code-to-unicode.html
// http://nihongo.monash.edu/coding_inf.htm
pub enum Codepoint {
    Jis208(Kuten),
    Jis212(Kuten),
    Jis213(PlanarKuten),
    Unicode(char),
}

pub struct Kuten {
    pub ku: u8,
    pub ten: u8,
}

pub struct PlanarKuten {
    pub plane: u8,
    pub ku: u8,
    pub ten: u8,
}

pub enum Radical {
    Classical(u8),
    Nelson(u8),
}

pub struct StrokeCount {
    pub accepted: u8,
    pub miscounts: Vec<u8>,
}

pub struct Grade(pub Option<u8>);