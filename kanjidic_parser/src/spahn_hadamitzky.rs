use crate::{
    pos_error::PosError,
    shared::{self, take_uint, IResult, NomErr, NomErrorReason, SharedError},
};
use kanjidic_types::ShDesc;
use nom::{bytes::complete::take, character::complete::char, sequence::tuple};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ShError {
    #[error("(Spahn Hadamitzky) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Spahn Hadamitzky) Parsing: {0}, {1}")]
    Parse(PosError, ShStrError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ShStrError {
    #[error("(Spahn Hadamitzky) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for ShStrError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

// They are in the form nxnn.n,
// e.g.  3k11.2, where the  kanji has 3 strokes in the
// identifying radical, it is radical "k" in the SH
// classification system, there are 11 other strokes, and it is
// the 2nd kanji in the 3k11 sequence.

fn parse_str_sh(text: &str) -> Result<ShDesc, ShError> {
    let (_i, (radical_strokes, radical, other_strokes, _, sequence)) = parts(text)?;
    let radical = radical.chars().next().unwrap();
    Ok(ShDesc {
        radical_strokes,
        radical,
        other_strokes,
        sequence,
    })
}

pub fn parse_sh(node: Node) -> Result<ShDesc, ShError> {
    let text = shared::text(node)?;
    parse_str_sh(text).map_err(|err| ShError::Parse(PosError::from(node), err))
}

fn parts(s: &str) -> IResult<(u8, &str, u8, char, u8)> {
    tuple((take_uint, take(1u8), take_uint, char('.'), take_uint))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_shared::DOC;

    #[test]
    fn spahn_hadamitzky() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("variant")
                    && node
                        .attribute("var_type")
                        .map(|value| value.eq("s_h"))
                        .unwrap_or(false)
            })
            .unwrap();
        let sh = ShDesc::try_from(node);
        assert_eq!(
            sh,
            Ok(ShDesc {
                radical_strokes: 2,
                radical: 'k',
                other_strokes: 4,
                sequence: 6,
            })
        )
    }
}
