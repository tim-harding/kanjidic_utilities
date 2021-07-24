use std::convert::TryFrom;

use crate::{
    busy_people::{BusyPeople, BusyPeopleError},
    moro::{Moro, MoroError},
};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DictionaryReferenceError {
    #[error("Node contains no text")]
    NoText,
    #[error("Node is missing dr_type attribute")]
    MissingType,
    #[error("Unknown dr_type string")]
    UnknownType,
    #[error("Could not parse element reference")]
    Numeric,
    #[error("Could not parse Moro reference")]
    Moro(#[from] MoroError),
    #[error("Could not parse Busy People reference")]
    BusyPeople(#[from] BusyPeopleError),
}

/// An index number into a particular kanji dictionary or reference book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DictionaryReference {
    /// Modern Reader's Japanese-English Dictionary by Andrew Nelson
    NelsonClassic(u16),

    /// The New Nelson Japanese-English Dictionary by John Haig
    NelsonNew(u16),

    /// New Japanese-English Character Dictionary by Jack Halpern
    Njecd(u16),

    /// Kodansha's Japanese-English Dictionary by Jack Halpern
    Kkd(u16),

    /// Kanji Learners Dictionary by Jack Halpern
    Kkld(u16),

    /// Kanji Learners Dictionary Second Edition by Jack Halpern
    Kkld2ed(u16),

    /// Remembering the Kanji by James Heisig
    Heisig(u16),

    /// Remembering the Kanji Sixth Edition by James Heisig
    Heisig6(u16),

    /// A New Dictionary of Kanji Usage
    Gakken(u16),

    /// Japanese Names by P.G. O'Neill
    OneillNames(u16),

    /// Essential Kanji by P.G. O'Neill
    OneillKk(u16),

    /// Daikanwajiten by Morohashi
    Moro(Moro),

    /// A Guide to Remembering Japanese Characters by Kenneth G. Henshall
    Henshall(u16),

    /// Kanji and Kana by Spahn and Hadamitzky
    ShKk(u16),

    /// Kanji and Kana 2011 edition by Spahn and Hadamitzky
    ShKk2(u16),

    /// A Guide to Reading and Writing Japanese by Florence Sakade
    Sakade(u16),

    /// Japanese Kanji Flashcards by Tomoko Okazaki
    Jfcards(u16),

    /// A Guide to Reading and Writing Japanese by Henshall
    Henshall3(u16),

    /// Tuttle Kanji Cards by Alexander Kask
    TuttleCards(u16),

    /// The Kanji Way to Japanese Language Power by Dale Crowley
    Crowley(u16),

    /// Kanji in Context by Nishiguchi and Kono
    KanjiInContext(u16),

    /// Japanese for Busy People
    BusyPeople(BusyPeople),

    /// The Kodansha Compact Study Guide
    KodanshaCompact(u16),

    /// Les Kanjis dans la tete by Yves Maniette
    Maniette(u16),
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for DictionaryReference {
    type Error = DictionaryReferenceError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let reference_type = node
            .attribute("dr_type")
            .ok_or(DictionaryReferenceError::MissingType)?;
        match reference_type {
            "nelson_c" => Ok(DictionaryReference::NelsonClassic(element(node)?)),
            "nelson_n" => Ok(DictionaryReference::NelsonNew(element(node)?)),
            "halpern_njecd" => Ok(DictionaryReference::Njecd(element(node)?)),
            "halpern_kkd" => Ok(DictionaryReference::Kkd(element(node)?)),
            "halpern_kkld" => Ok(DictionaryReference::Kkld(element(node)?)),
            "halpern_kkld_2ed" => Ok(DictionaryReference::Kkld2ed(element(node)?)),
            "heisig" => Ok(DictionaryReference::Heisig(element(node)?)),
            "heisig6" => Ok(DictionaryReference::Heisig6(element(node)?)),
            "gakken" => Ok(DictionaryReference::Gakken(element(node)?)),
            "oneill_names" => Ok(DictionaryReference::OneillNames(element(node)?)),
            "oneill_kk" => Ok(DictionaryReference::OneillKk(element(node)?)),
            "moro" => Ok(DictionaryReference::Moro(Moro::try_from(node)?)),
            "henshall" => Ok(DictionaryReference::Henshall(element(node)?)),
            "sh_kk" => Ok(DictionaryReference::ShKk(element(node)?)),
            "sh_kk2" => Ok(DictionaryReference::ShKk2(element(node)?)),
            "sakade" => Ok(DictionaryReference::Sakade(element(node)?)),
            "jf_cards" => Ok(DictionaryReference::Jfcards(element(node)?)),
            "henshall3" => Ok(DictionaryReference::Henshall3(element(node)?)),
            "tutt_cards" => Ok(DictionaryReference::TuttleCards(element(node)?)),
            "crowley" => Ok(DictionaryReference::Crowley(element(node)?)),
            "kanji_in_context" => Ok(DictionaryReference::KanjiInContext(element(node)?)),
            "busy_people" => Ok(DictionaryReference::BusyPeople(BusyPeople::try_from(node)?)),
            "kodansha_compact" => Ok(DictionaryReference::KodanshaCompact(element(node)?)),
            "maniette" => Ok(DictionaryReference::Maniette(element(node)?)),
            _ => Err(DictionaryReferenceError::UnknownType),
        }
    }
}

fn element(node: Node) -> Result<u16, DictionaryReferenceError> {
    let text = node.text().ok_or(DictionaryReferenceError::NoText)?;
    text.parse().map_err(|_| DictionaryReferenceError::Numeric)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn dictionary_reference() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("dic_ref"))
            .unwrap();
        let dictionary_reference = DictionaryReference::try_from(node);
        assert_eq!(
            dictionary_reference,
            Ok(DictionaryReference::NelsonClassic(43))
        )
    }
}
