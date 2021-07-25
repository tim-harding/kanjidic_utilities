use std::convert::TryFrom;

use nom::{
    bytes::complete::take_while,
    combinator::{map, map_res},
    sequence::tuple,
};
use roxmltree::Node;
use thiserror::Error;

use crate::shared::{attr_uint, take_uint, text, IResult, NomErr, NomErrorReason, SharedError};

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum MoroError {
    #[error("(Moro) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Moro) Unknown index suffix")]
    IndexSuffix,
    #[error("(Moro) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for MoroError {
    fn from(err: NomErr<'a>) -> Self {
        MoroError::Format(err.into())
    }
}

/// An entry in the dictionary Daikanwajiten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Moro {
    /// The volume
    pub volume: Option<u8>,
    /// The page
    pub page: Option<u16>,
    /// The reference index
    pub index: MoroIndex,
}

/// The reference index
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MoroIndex {
    /// The item number
    pub number: u16,
    /// A letter that appears after the index
    pub suffix: MoroSuffix,
}

/// A letter that appears at the end of the index
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MoroSuffix {
    /// No suffix
    None,
    /// P suffix
    P,
    /// X suffix
    X,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Moro {
    type Error = MoroError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let (_i, index) = parse_index(text(node)?)?;
        let volume = attr_uint::<u8>(node, "m_vol")?;
        let page = attr_uint::<u16>(node, "m_page")?;
        Ok(Moro {
            volume,
            page,
            index,
        })
    }
}

fn parse_index(s: &str) -> IResult<MoroIndex> {
    map(parts, |parts| {
        let (number, suffix) = parts;
        MoroIndex { number, suffix }
    })(s)
}

fn parts(s: &str) -> IResult<(u16, MoroSuffix)> {
    tuple((take_uint, index_suffix))(s)
}

fn index_suffix(s: &str) -> IResult<MoroSuffix> {
    map_res(
        take_while(|c: char| c.is_ascii_alphabetic()),
        |text| match text {
            "X" => Ok(MoroSuffix::X),
            "P" => Ok(MoroSuffix::P),
            "" => Ok(MoroSuffix::None),
            _ => Err(MoroError::IndexSuffix),
        },
    )(s)
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
                index: MoroIndex {
                    number: 272,
                    suffix: MoroSuffix::None,
                },
            })
        )
    }
}
