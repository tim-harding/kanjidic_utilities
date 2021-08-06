use kanjidic_types::Kuten;
use nom::character::complete::char;
use nom::sequence::tuple;
use roxmltree::Node;
use thiserror::Error;

use crate::{
    pos_error::PosError,
    shared::{take_uint, text, IResult, NomErr, NomErrorReason, SharedError},
};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KutenError {
    #[error("(Kuten) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Kuten) Parsing: {0}, {1}")]
    Parse(PosError, KutenStrError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KutenStrError {
    #[error("(Kuten) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for KutenStrError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

pub fn from(node: Node) -> Result<Kuten, KutenError> {
    from_str(text(node)?).map_err(|err| KutenError::Parse(PosError::from(node), err))
}

pub fn from_str(text: &str) -> Result<Kuten, KutenStrError> {
    let (_i, o) = kuten_parts(text)?;
    let (plane, _, ku, _, ten) = o;
    Ok(Kuten { plane, ku, ten })
}

fn kuten_parts(s: &str) -> IResult<(u8, char, u8, char, u8)> {
    tuple((take_uint, char('-'), take_uint, char('-'), take_uint))(s)
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::Kuten;

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
        let kuten = from(node);
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
