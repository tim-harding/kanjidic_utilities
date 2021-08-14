use kanjidic_types::Character;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

pub type KanjiCache = HashMap<String, Character>;
pub type RadkCache = HashMap<String, Radk>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Radk {
    pub radical: String,
    pub stroke: u8,
    pub kanji: HashSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cache {
    pub kanji: KanjiCache,
    pub radk: RadkCache,
}
