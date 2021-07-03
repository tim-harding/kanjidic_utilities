use crate::{kunyomi::Kunyomi, pin_yin::PinYin};

pub enum Reading<'a> {
    PinYin(PinYin<'a>),
    KoreanRomanized(&'a str),
    KoreanHangul(&'a str),
    Vietnam(&'a str),
    Onyomi(&'a str),
    Kunyomi(Kunyomi<'a>),
}
