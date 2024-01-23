use crate::{
    pos_error::PosError,
    shared::{attr, text_uint, SharedError},
};
use kanjidic_types::{radical::RadicalKind, KangXi, Radical, TryFromPrimitiveError};
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, thiserror::Error)]
pub enum Error {
    #[error("(Radical) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Radical) Radical is not in a valid range: {0}")]
    OutOfRange(#[from] TryFromPrimitiveError<KangXi>),
    #[error("(Radical) Not a recognized radical kind: {0}")]
    Kind(PosError),
}

pub fn from(node: Node) -> Result<Radical, Error> {
    let kang_xi_number: u8 = text_uint(&node)?;
    let kang_xi = KangXi::try_from(kang_xi_number)?;
    let tag = attr(&node, "rad_type")?;
    match tag {
        "classical" => Ok(Radical {
            kind: RadicalKind::Classical,
            radical: kang_xi,
        }),
        "nelson_c" => Ok(Radical {
            kind: RadicalKind::Nelson,
            radical: kang_xi,
        }),
        _ => Err(Error::Kind(PosError::from(&node))),
    }
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{radical::RadicalKind, KangXi, Radical};

    #[test]
    fn parse_radical() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("rad_value"))
            .unwrap();
        let radical = from(node);
        assert_eq!(
            radical,
            Ok(Radical {
                kind: RadicalKind::Classical,
                radical: KangXi::Two,
            })
        )
    }
}
