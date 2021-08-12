use std::collections::{HashMap, HashSet};
use kanjidic_types::Character;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KanjiData {
    pub character: Character,
    pub decomposition: Option<HashSet<String>>,
}

pub type KanjiCache = HashMap<String, KanjiData>;
pub type RadkCache = HashMap<String, HashSet<String>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
