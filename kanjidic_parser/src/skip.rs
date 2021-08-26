use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};
use kanjidic_types::{Skip, SkipStrError};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SkipError {
    #[error("(Skip) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Skip) Parsing: {0}, {1}")]
    Parse(PosError, SkipStrError),
}

pub fn from(node: Node) -> Result<Skip, SkipError> {
    let text = shared::text(node)?;
    Skip::try_from(text).map_err(|err| SkipError::Parse(PosError::from(node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{Skip, SkipSolid, SolidSubpattern};

    #[test]
    fn skip() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("q_code")
                    && node
                        .attribute("qc_type")
                        .map(|value| value.eq("skip"))
                        .unwrap_or(false)
            })
            .unwrap();
        let skip = from(node);
        assert_eq!(
            skip,
            Ok(Skip::Solid(SkipSolid {
                total_stroke_count: 7,
                solid_subpattern: SolidSubpattern::TopLine,
            }))
        )
    }
}
