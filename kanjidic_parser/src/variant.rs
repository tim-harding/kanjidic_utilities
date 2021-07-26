use crate::{de_roo::{DeRoo, DeRooError}, kuten::{Kuten, KutenError}, oneill::{Oneill, OneillError}, pos_error::PosError, shared::{SharedError, attr, text_hex, text_uint}, spahn_hadamitzky::{ShDesc, ShError}};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;
use serde::{Serialize, Deserialize};

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

/// Represents either of the following:
/// - A cross-reference to another kanji usually regarded as a variant
/// - An alternative indexing code for the current kanji
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Variant {
    /// A coding in JIS X 0208
    Jis208(Kuten),
    /// A coding in JIS X 0212
    Jis212(Kuten),
    /// A coding in JIS X 0213
    Jis213(Kuten),
    /// A unicode codepoint
    Unicode(u32),
    /// An identification in the De Roo system
    DeRoo(DeRoo),
    /// Index in the NJECD system.
    Halpern(u16),
    /// The Kanji Dictionary kanji code.
    Sh(ShDesc),
    /// Index in the Modern Reader's Japanese-English dictionary.
    Nelson(u16),
    /// Index in Japanese Names by P.G. O'Neill.
    ONeill(Oneill),
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Variant {
    type Error = VariantError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let variant_type = attr(node, "var_type")?;
        match variant_type {
            "jis208" => Ok(Variant::Jis208(Kuten::try_from(node)?)),
            "jis212" => Ok(Variant::Jis212(Kuten::try_from(node)?)),
            "jis213" => Ok(Variant::Jis213(Kuten::try_from(node)?)),
            "deroo" => Ok(Variant::DeRoo(DeRoo::try_from(node)?)),
            "njecd" => Ok(Variant::Halpern(text_uint::<u16>(node)?)),
            "s_h" => Ok(Variant::Sh(ShDesc::try_from(node)?)),
            "nelson_c" => Ok(Variant::Nelson(text_uint::<u16>(node)?)),
            "oneill" => Ok(Variant::ONeill(Oneill::try_from(node)?)),
            "ucs" => Ok(Variant::Unicode(text_hex(node)?)),
            _ => Err(VariantError::UnknownVariant(PosError::from(node))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn variant() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("variant"))
            .unwrap();
        let variant = Variant::try_from(node);
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
