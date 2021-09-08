use kanjidic_types::Character;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

pub type KanjiCache = HashMap<char, Character>;
pub type RadkCache = HashMap<char, Radk>;
pub type TranslationCache = HashMap<String, Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Radk {
    pub radical: char,
    pub stroke: u8,
    pub kanji: HashSet<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cache {
    pub kanji: KanjiCache,
    pub radk: RadkCache,
    pub translations: TranslationCache,
}
