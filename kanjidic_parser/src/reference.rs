use crate::{
    busy_people, moro, oneill,
    pos_error::PosError,
    shared::{attr, text_uint, SharedError},
    BusyPeopleError, MoroError, OneillError,
};
use kanjidic_types::Reference;
use roxmltree::Node;
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

pub fn from(node: Node) -> Result<Reference, ReferenceError> {
    match attr(node, "dr_type")? {
        "nelson_c" => Ok(Reference::NelsonClassic {
            index: text_uint(node)?,
        }),
        "nelson_n" => Ok(Reference::NelsonNew {
            index: text_uint(node)?,
        }),
        "halpern_njecd" => Ok(Reference::Njecd {
            index: text_uint(node)?,
        }),
        "halpern_kkd" => Ok(Reference::Kkd {
            index: text_uint(node)?,
        }),
        "halpern_kkld" => Ok(Reference::Kkld {
            index: text_uint(node)?,
        }),
        "halpern_kkld_2ed" => Ok(Reference::Kkld2ed {
            index: text_uint(node)?,
        }),
        "heisig" => Ok(Reference::Heisig {
            index: text_uint(node)?,
        }),
        "heisig6" => Ok(Reference::Heisig6 {
            index: text_uint(node)?,
        }),
        "gakken" => Ok(Reference::Gakken {
            index: text_uint(node)?,
        }),
        "oneill_names" => Ok(Reference::OneillNames(oneill::from(node)?)),
        "oneill_kk" => Ok(Reference::OneillKk {
            index: text_uint(node)?,
        }),
        "moro" => Ok(Reference::Moro(moro::from(node)?)),
        "henshall" => Ok(Reference::Henshall {
            index: text_uint(node)?,
        }),
        "sh_kk" => Ok(Reference::ShKk {
            index: text_uint(node)?,
        }),
        "sh_kk2" => Ok(Reference::ShKk2 {
            index: text_uint(node)?,
        }),
        "sakade" => Ok(Reference::Sakade {
            index: text_uint(node)?,
        }),
        "jf_cards" => Ok(Reference::Jfcards {
            index: text_uint(node)?,
        }),
        "henshall3" => Ok(Reference::Henshall3 {
            index: text_uint(node)?,
        }),
        "tutt_cards" => Ok(Reference::TuttleCards {
            index: text_uint(node)?,
        }),
        "crowley" => Ok(Reference::Crowley {
            index: text_uint(node)?,
        }),
        "kanji_in_context" => Ok(Reference::KanjiInContext {
            index: text_uint(node)?,
        }),
        "busy_people" => Ok(Reference::BusyPeople(busy_people::from(node)?)),
        "kodansha_compact" => Ok(Reference::KodanshaCompact {
            index: text_uint(node)?,
        }),
        "maniette" => Ok(Reference::Maniette {
            index: text_uint(node)?,
        }),
        _ => Err(ReferenceError::UnknownType(PosError::from(node))),
    }
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::Reference;

    #[test]
    fn dictionary_reference() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("dic_ref"))
            .unwrap();
        let dictionary_reference = from(node);
        assert_eq!(
            dictionary_reference,
            Ok(Reference::NelsonClassic { index: 43 })
        )
    }
}
