use crate::{codepoint::Codepoint, radical::Radical, stroke_count::StrokeCount, variant::Variant};

pub struct Character<'a> {
    pub literal: &'a str,
    pub codepoints: Vec<Codepoint>,
    pub radicals: Vec<Radical>,
    pub grade: Option<u8>,
    pub stroke_count: StrokeCount,
    pub variants: Vec<Variant>,
    pub radical_names: Vec<&'a str>,
    pub jlpt: Option<u8>,
    pub frequency: Option<u16>,
}
