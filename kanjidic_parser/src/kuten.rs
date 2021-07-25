use nom::character::complete::char;
use nom::sequence::tuple;
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

use crate::{
    pos_error::PosError,
    shared::{take_uint, text, IResult, SharedError},
};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KutenError {
    #[error("Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("Error parsing kuten: {0}, {1}")]
    Str(PosError, KutenStrError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KutenStrError {
    #[error("Failed to parse kuten")]
    Parse,
}

/// A kuten representation of a JIS X 0213 character.
/// http://unicode-iphone.blogspot.com/2010/05/kuten-code-to-unicode.html
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct Kuten {
    /// The plane on which a kuten representation is found.
    pub plane: u8,
    /// The Ku part of the matrix position.
    pub ku: u8,
    /// The Ten part of the matrix position.
    pub ten: u8,
}

impl TryFrom<&str> for Kuten {
    type Error = KutenStrError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, o) = kuten_parts(text).map_err(|_| KutenStrError::Parse)?;
        let (plane, _, ku, _, ten) = o;
        Ok(Self { plane, ku, ten })
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Kuten {
    type Error = KutenError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        Self::try_from(text(node)?).map_err(|err| KutenError::Str(PosError::from(node), err))
    }
}

fn kuten_parts(s: &str) -> IResult<(u8, char, u8, char, u8)> {
    tuple((take_uint, char('-'), take_uint, char('-'), take_uint))(s)
}

#[cfg(test)]
mod tests {
    use crate::{kuten::Kuten, test_shared::DOC};
    use std::convert::TryFrom;

    #[test]
    fn parse_kuten() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("cp_value")
                    && node
                        .attribute("cp_type")
                        .map(|value| value.eq("jis208"))
                        .unwrap_or(false)
            })
            .unwrap();
        let kuten = Kuten::try_from(node);
        assert_eq!(
            kuten,
            Ok(Kuten {
                plane: 1,
                ku: 16,
                ten: 1,
            })
        )
    }
}
