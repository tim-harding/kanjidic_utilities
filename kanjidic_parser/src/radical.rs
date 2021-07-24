use crate::kangxi::KangXi;
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum RadicalError {
    #[error("Node contains no text")]
    NoText,
    #[error("No attribute specifying radical type")]
    NoType,
    #[error("The radical is not in a valid range")]
    OutOfRange,
    #[error("Could not parse text content as a number")]
    Number,
    #[error("Not a recognized radical kind")]
    Kind,
}

/// A kanji classification based on its radical.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum Radical {
    /// Based on the KangXi Zidian system.
    /// Referenced from the Shibano JIS Kanwa Jiten.
    Classical(KangXi),

    /// As used in the classic Modern Japanese-English Character Dictionary.
    Nelson(KangXi),
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Radical {
    type Error = RadicalError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = node.text().ok_or(RadicalError::NoText)?;
        let kang_xi_number: u8 = text.parse().map_err(|_| RadicalError::Number)?;
        let kang_xi = KangXi::try_from(kang_xi_number).map_err(|_| RadicalError::OutOfRange)?;
        let tag = node.attribute("rad_type").ok_or(RadicalError::NoType)?;
        match tag {
            "classical" => Ok(Radical::Classical(kang_xi)),
            "nelson_c" => Ok(Radical::Nelson(kang_xi)),
            _ => Err(RadicalError::Kind),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Radical;
    use crate::{kangxi::KangXi, test_shared::DOC};
    use std::convert::TryFrom;

    #[test]
    fn parse_radical() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("rad_value"))
            .unwrap();
        let radical = Radical::try_from(node);
        assert_eq!(radical, Ok(Radical::Classical(KangXi::Two)))
    }
}
