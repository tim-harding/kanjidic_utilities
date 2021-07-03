use crate::{reading::Reading, translation::Translation};

pub struct Meaning<'a> {
    pub readings: Vec<Reading<'a>>,
    pub translations: Vec<Translation<'a>>,
    pub nanori: Vec<&'a str>,
}
