use kanjidic_types::Character;
use std::collections::{HashMap, HashSet};

pub type KanjiCache = HashMap<String, Character>;
pub type RadkCache = HashMap<String, HashSet<String>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cache {
    pub kanji: KanjiCache,
    pub radk: RadkCache,
}
