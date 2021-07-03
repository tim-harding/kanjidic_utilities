use iso639_1::Iso639_1;

pub struct Translation<'a> {
    pub text: &'a str,
    pub language: Iso639_1,
}
