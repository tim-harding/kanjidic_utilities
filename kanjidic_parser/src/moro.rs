use std::{convert::TryFrom, str::FromStr};

use roxmltree::Node;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum MoroError {
    #[error("Node contains no text")]
    NoText,
    #[error("Could not parse the number")]
    Numeric,
}

/// An entry in the dictionary Daikanwajiten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Moro {
    /// The volume
    pub volume: Option<u8>,

    /// The page
    pub page: Option<u16>,

    /// The item number
    pub item: u16,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Moro {
    type Error = MoroError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = node.text().ok_or(MoroError::NoText)?;
        let item = u16::from_str_radix(text, 10).map_err(|_| MoroError::Numeric)?;
        let volume = numeric_attribute::<u8>(node, "m_vol")?;
        let page = numeric_attribute::<u16>(node, "m_page")?;
        Ok(Moro { volume, page, item })
    }
}

fn numeric_attribute<T: FromStr>(
    node: Node,
    attribute: &'static str,
) -> Result<Option<T>, MoroError> {
    match node.attribute(attribute) {
        Some(text) => {
            let parsed: T = text.parse().map_err(|_| MoroError::Numeric)?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn pin_yin() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("dic_ref")
                    && node
                        .attribute("dr_type")
                        .map(|value| value.eq("moro"))
                        .unwrap_or(false)
            })
            .unwrap();
        let moro = Moro::try_from(node);
        assert_eq!(
            moro,
            Ok(Moro {
                volume: Some(1),
                page: Some(525),
                item: 272,
            })
        )
    }
}
