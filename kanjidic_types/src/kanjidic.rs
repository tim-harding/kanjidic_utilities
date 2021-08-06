use serde::{Deserialize, Serialize};

use crate::{Character, Header};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Kanjidic {
    pub header: Header,
    pub characters: Vec<Character>,
}
