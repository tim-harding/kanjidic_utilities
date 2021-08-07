use crate::{
    de_roo::{self, DeRooError},
    kuten, oneill,
    pos_error::PosError,
    shared::{attr, text_hex, text_uint, SharedError},
    spahn_hadamitzky::{self, ShError},
    KutenError, OneillError,
};
use kanjidic_types::Variant;
use roxmltree::Node;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum VariantError {
    #[error("(Variant) var_type not recognized: {0}")]
    UnknownVariant(PosError),
    #[error("(Variant) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Variant) Kuten code: {0}")]
    Kuten(#[from] KutenError),
    #[error("(Variant) De Roo code: {0}")]
    DeRoo(#[from] DeRooError),
    #[error("(Variant) Spahn Hadamitzky descriptor: {0}")]
    SpahnHadamitzky(#[from] ShError),
    #[error("(Variant) ONeill: {0}")]
    ONeill(#[from] OneillError),
}

pub fn from(node: Node) -> Result<Variant, VariantError> {
    let variant_type = attr(node, "var_type")?;
    match variant_type {
        "jis208" => Ok(Variant::Jis208(kuten::from(node)?)),
        "jis212" => Ok(Variant::Jis212(kuten::from(node)?)),
        "jis213" => Ok(Variant::Jis213(kuten::from(node)?)),
        "deroo" => Ok(Variant::DeRoo(de_roo::from(node)?)),
        "njecd" => Ok(Variant::Halpern {
            code: text_uint::<u16>(node)?,
        }),
        "s_h" => Ok(Variant::Sh(spahn_hadamitzky::from(node)?)),
        "nelson_c" => Ok(Variant::Nelson {
            code: text_uint::<u16>(node)?,
        }),
        "oneill" => Ok(Variant::ONeill(oneill::from(node)?)),
        "ucs" => Ok(Variant::Unicode {
            code: text_hex(node)?,
        }),
        _ => Err(VariantError::UnknownVariant(PosError::from(node))),
    }
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{Kuten, Variant};

    #[test]
    fn variant() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("variant"))
            .unwrap();
        let variant = from(node);
        assert_eq!(
            variant,
            Ok(Variant::Jis208(Kuten {
                plane: 1,
                ku: 48,
                ten: 19,
            }))
        )
    }
}
