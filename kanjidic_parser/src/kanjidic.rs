use crate::{
    character,
    header::{self, Header},
};
use kanjidic_types::Character;
use roxmltree::{Document, Node};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("Error parsing XML file")]
    Xml(#[from] roxmltree::Error),
    #[error("Could not find header node")]
    MissingHeader,
    #[error("Error parsing database version")]
    Header(#[from] header::Error),
    #[error("Error parsing a character")]
    Character(#[from] character::CharacterError),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Kanjidic {
    pub header: Header,
    pub characters: Vec<Character>,
}

impl<'a> TryFrom<&'a Document<'a>> for Kanjidic {
    type Error = Error;

    fn try_from(doc: &'a Document) -> Result<Self, Self::Error> {
        let root = doc.root_element();
        let header = Header::try_from(
            root.children()
                .find(|child| child.has_tag_name("header"))
                .ok_or(Error::MissingHeader)?,
        )?;
        let characters: Result<Vec<Character>, character::CharacterError> = root
            .children()
            .filter(|child| child.has_tag_name("character"))
            .collect::<Vec<Node>>()
            .iter()
            .map(|node| character::from(*node))
            .collect::<Result<Vec<Character>, character::CharacterError>>();
        let characters = characters?;
        Ok(Self { header, characters })
    }
}

impl TryFrom<&str> for Kanjidic {
    type Error = Error;

    fn try_from(xml: &str) -> Result<Self, Self::Error> {
        let doc = roxmltree::Document::parse(xml)?;
        Self::try_from(&doc)
    }
}

#[cfg(test)]
mod tests {
    use super::Kanjidic;
    use crate::test_shared::DOC;
    use roxmltree::Document;
    use std::convert::TryFrom;

    #[test]
    #[ignore]
    fn kanjidic() {
        let _kanjidic = Kanjidic::try_from(&DOC as &Document).unwrap();
    }
}
