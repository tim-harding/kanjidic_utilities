mod busy_people;
mod character;
mod codepoint;
mod database_version;
mod date_of_creation;
mod de_roo;
mod dictionary_reference;
mod four_corner;
mod grade;
mod header;
mod kangxi;
mod kunyomi;
mod kuten;
mod meaning;
mod moro;
mod oneill;
mod pin_yin;
mod pos_error;
mod query_code;
mod radical;
mod reading;
mod shared;
mod skip;
mod spahn_hadamitzky;
mod stroke_count;
mod translation;
mod variant;

#[cfg(test)]
mod test_shared;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use std::convert::TryFrom;

use character::{Character, CharacterError};
use header::{Header, HeaderError};
use rayon::prelude::*;
use roxmltree::{Document, Node};
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Error)]
pub enum KanjidicError {
    #[error("Error parsing XML file")]
    Xml(#[from] roxmltree::Error),
    #[error("Could not find header node")]
    MissingHeader,
    #[error("Error parsing database version")]
    Header(#[from] HeaderError),
    #[error("Error parsing a character")]
    Character(#[from] CharacterError),
}

pub struct KanjidicDocument<'input> {
    doc: Document<'input>,
}

impl<'input> KanjidicDocument<'input> {
    pub fn new(xml: &'input str) -> Result<Self, KanjidicError> {
        let doc = roxmltree::Document::parse(xml)?;
        Ok(Self { doc })
    }

    pub fn kanjidic<'a, 'b: 'a>(&'b self) -> Result<Kanjidic, KanjidicError> {
        Kanjidic::new(&self.doc)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kanjidic {
    pub header: Header,
    pub characters: Vec<Character>,
}

impl<'a, 'b: 'a> Kanjidic {
    fn new(doc: &'b Document) -> Result<Kanjidic, KanjidicError> {
        let root = doc.root_element();
        let header = Header::try_from(
            root.children()
                .find(|child| child.has_tag_name("header"))
                .ok_or(KanjidicError::MissingHeader)?,
        )?;
        let characters: Result<Vec<Character>, CharacterError> = root
            .children()
            .filter(|child| child.has_tag_name("character"))
            .collect::<Vec<Node>>()
            .par_iter()
            .map(|node| Character::try_from(node.clone()))
            .collect::<Result<Vec<Character>, CharacterError>>();
        let characters = characters?;
        Ok(Kanjidic { header, characters })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn kanjidic() {
        let kanjidic = Kanjidic::new(&DOC).unwrap();
        println!("{:?}", kanjidic);
    }
}
