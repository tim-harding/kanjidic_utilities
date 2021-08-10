use kanjidic_types::Character;
use rayon::prelude::*;
use roxmltree::{Document, Node};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

use crate::{
    character,
    header::{Header, HeaderError},
    CharacterError,
};

#[derive(Debug, Error, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Kanjidic {
    pub header: Header,
    pub characters: Vec<Character>,
}

impl<'a> TryFrom<&'a Document<'a>> for Kanjidic {
    type Error = KanjidicError;

    fn try_from(doc: &'a Document) -> Result<Self, Self::Error> {
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
            .map(|node| character::from(*node))
            .collect::<Result<Vec<Character>, CharacterError>>();
        let characters = characters?;
        Ok(Self { header, characters })
    }
}

impl TryFrom<&str> for Kanjidic {
    type Error = KanjidicError;

    fn try_from(xml: &str) -> Result<Self, Self::Error> {
        let doc = roxmltree::Document::parse(xml)?;
        Self::try_from(&doc)
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_shared::DOC, Kanjidic};
    use roxmltree::Document;
    use std::convert::TryFrom;

    #[test]
    #[ignore]
    fn kanjidic() {
        let _kanjidic = Kanjidic::try_from(&DOC as &Document).unwrap();
    }
}
