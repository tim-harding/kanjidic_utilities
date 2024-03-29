use crate::{
    busy_people, moro, oneill,
    pos_error::PosError,
    shared::{attr, text_uint, SharedError},
};
use kanjidic_types::Reference;
use roxmltree::Node;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("(Reference) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Reference) Unknown dr_type string: {0}")]
    UnknownType(PosError),
    #[error("(Reference) Moro: {0}")]
    Moro(#[from] moro::Error),
    #[error("(Reference) Busy People: {0}")]
    BusyPeople(#[from] busy_people::Error),
    #[error("(Reference) Oneill: {0}")]
    Oneill(#[from] oneill::Error),
}

pub fn from(node: Node) -> Result<Reference, Error> {
    match attr(&node, "dr_type")? {
        "nelson_c" => Ok(Reference::NelsonClassic(text_uint(&node)?)),
        "nelson_n" => Ok(Reference::NelsonNew(text_uint(&node)?)),
        "halpern_njecd" => Ok(Reference::Njecd(text_uint(&node)?)),
        "halpern_kkd" => Ok(Reference::Kkd(text_uint(&node)?)),
        "halpern_kkld" => Ok(Reference::Kkld(text_uint(&node)?)),
        "halpern_kkld_2ed" => Ok(Reference::Kkld2ed(text_uint(&node)?)),
        "heisig" => Ok(Reference::Heisig(text_uint(&node)?)),
        "heisig6" => Ok(Reference::Heisig6(text_uint(&node)?)),
        "gakken" => Ok(Reference::Gakken(text_uint(&node)?)),
        "oneill_names" => Ok(Reference::OneillNames(oneill::from(node)?)),
        "oneill_kk" => Ok(Reference::OneillKk(text_uint(&node)?)),
        "moro" => Ok(Reference::Moro(moro::from(node)?)),
        "henshall" => Ok(Reference::Henshall(text_uint(&node)?)),
        "sh_kk" => Ok(Reference::ShKk(text_uint(&node)?)),
        "sh_kk2" => Ok(Reference::ShKk2(text_uint(&node)?)),
        "sakade" => Ok(Reference::Sakade(text_uint(&node)?)),
        "jf_cards" => Ok(Reference::Jfcards(text_uint(&node)?)),
        "henshall3" => Ok(Reference::Henshall3(text_uint(&node)?)),
        "tutt_cards" => Ok(Reference::TuttleCards(text_uint(&node)?)),
        "crowley" => Ok(Reference::Crowley(text_uint(&node)?)),
        "kanji_in_context" => Ok(Reference::KanjiInContext(text_uint(&node)?)),
        "busy_people" => Ok(Reference::BusyPeople(busy_people::from(node)?)),
        "kodansha_compact" => Ok(Reference::KodanshaCompact(text_uint(&node)?)),
        "maniette" => Ok(Reference::Maniette(text_uint(&node)?)),
        _ => Err(Error::UnknownType(PosError::from(&node))),
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
        assert_eq!(dictionary_reference, Ok(Reference::NelsonClassic(43)))
    }
}
