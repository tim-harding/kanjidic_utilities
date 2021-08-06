use std::convert::TryFrom;

use rayon::prelude::*;
use roxmltree::{Document, Node};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    character::{Character, CharacterError},
    header::{Header, HeaderError},
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
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
            .map(|node| Character::try_from(*node))
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
    use roxmltree::Document;

    use crate::{test_shared::DOC, Kanjidic};
    use std::convert::TryFrom;

    #[test]
    fn kanjidic() {
        let _kanjidic = Kanjidic::try_from(&DOC as &Document).unwrap();
    }
}
