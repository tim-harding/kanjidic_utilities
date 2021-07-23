use nom::character::complete::char;
use nom::sequence::tuple;
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

use crate::shared::{digit, IResult};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KutenError {
    #[error("Failed to parse kuten")]
    Parse,
    #[error("Node contained no text")]
    NoText,
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

impl<'a> TryFrom<&'a str> for Kuten {
    type Error = KutenError;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let (_i, o) = kuten_parts(text).map_err(|_| KutenError::Parse)?;
        let (plane, _, ku, _, ten) = o;
        Ok(Self { plane, ku, ten })
    }
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Kuten {
    type Error = KutenError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let text = node.text().ok_or(KutenError::NoText)?;
        Self::try_from(text)
    }
}

fn kuten_parts(s: &str) -> IResult<(u8, char, u8, char, u8)> {
    tuple((digit, char('-'), digit, char('-'), digit))(s)
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
