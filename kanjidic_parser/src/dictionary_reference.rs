use std::convert::TryFrom;

use crate::{
    busy_people::{BusyPeople, BusyPeopleError},
    moro::{Moro, MoroError},
    oneill::{Oneill, OneillError},
    pos_error::PosError,
    shared::{attr, text_uint, SharedError},
};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ReferenceError {
    #[error("(Reference) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Reference) Unknown dr_type string: {0}")]
    UnknownType(PosError),
    #[error("(Reference) Moro: {0}")]
    Moro(#[from] MoroError),
    #[error("(Reference) Busy People: {0}")]
    BusyPeople(#[from] BusyPeopleError),
    #[error("(Reference) Oneill: {0}")]
    Oneill(#[from] OneillError),
}

/// An index number into a particular kanji dictionary or reference book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Reference {
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
    OneillNames(Oneill),
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

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Reference {
    type Error = ReferenceError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        match attr(node, "dr_type")? {
            "nelson_c" => Ok(Reference::NelsonClassic(text_uint(node)?)),
            "nelson_n" => Ok(Reference::NelsonNew(text_uint(node)?)),
            "halpern_njecd" => Ok(Reference::Njecd(text_uint(node)?)),
            "halpern_kkd" => Ok(Reference::Kkd(text_uint(node)?)),
            "halpern_kkld" => Ok(Reference::Kkld(text_uint(node)?)),
            "halpern_kkld_2ed" => Ok(Reference::Kkld2ed(text_uint(node)?)),
            "heisig" => Ok(Reference::Heisig(text_uint(node)?)),
            "heisig6" => Ok(Reference::Heisig6(text_uint(node)?)),
            "gakken" => Ok(Reference::Gakken(text_uint(node)?)),
            "oneill_names" => Ok(Reference::OneillNames(Oneill::try_from(node)?)),
            "oneill_kk" => Ok(Reference::OneillKk(text_uint(node)?)),
            "moro" => Ok(Reference::Moro(Moro::try_from(node)?)),
            "henshall" => Ok(Reference::Henshall(text_uint(node)?)),
            "sh_kk" => Ok(Reference::ShKk(text_uint(node)?)),
            "sh_kk2" => Ok(Reference::ShKk2(text_uint(node)?)),
            "sakade" => Ok(Reference::Sakade(text_uint(node)?)),
            "jf_cards" => Ok(Reference::Jfcards(text_uint(node)?)),
            "henshall3" => Ok(Reference::Henshall3(text_uint(node)?)),
            "tutt_cards" => Ok(Reference::TuttleCards(text_uint(node)?)),
            "crowley" => Ok(Reference::Crowley(text_uint(node)?)),
            "kanji_in_context" => Ok(Reference::KanjiInContext(text_uint(node)?)),
            "busy_people" => Ok(Reference::BusyPeople(BusyPeople::try_from(node)?)),
            "kodansha_compact" => Ok(Reference::KodanshaCompact(text_uint(node)?)),
            "maniette" => Ok(Reference::Maniette(text_uint(node)?)),
            _ => Err(ReferenceError::UnknownType(PosError::from(node))),
        }
    }
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
        let dictionary_reference = Reference::try_from(node);
        assert_eq!(dictionary_reference, Ok(Reference::NelsonClassic(43)))
    }
}
