use crate::{
    kangxi::KangXi,
    pos_error::PosError,
    shared::{attr, text_uint, SharedError},
};
use kanjidic_types::{TryFromPrimitiveError, Radical};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum RadicalError {
    #[error("(Radical) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Radical) Radical is not in a valid range: {0}")]
    OutOfRange(#[from] TryFromPrimitiveError<KangXi>),
    #[error("(Radical) Not a recognized radical kind: {0}")]
    Kind(PosError),
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Radical {
    type Error = RadicalError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let kang_xi_number: u8 = text_uint(node)?;
        let kang_xi = KangXi::try_from(kang_xi_number)?;
        let tag = attr(node, "rad_type")?;
        match tag {
            "classical" => Ok(Radical::Classical(kang_xi)),
            "nelson_c" => Ok(Radical::Nelson(kang_xi)),
            _ => Err(RadicalError::Kind(PosError::from(node))),
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
