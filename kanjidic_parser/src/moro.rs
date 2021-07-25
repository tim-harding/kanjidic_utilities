use std::convert::TryFrom;

use roxmltree::Node;
use thiserror::Error;

use crate::shared::{attr_uint, text_uint, SharedError};

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum MoroError {
    #[error("(Moro) Shared: {0}")]
    Shared(#[from] SharedError),
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
        let item: u16 = text_uint(node)?;
        let volume = attr_uint::<u8>(node, "m_vol")?;
        let page = attr_uint::<u16>(node, "m_page")?;
        Ok(Moro { volume, page, item })
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
