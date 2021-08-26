use std::convert::TryFrom;

use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};
use kanjidic_types::{FourCorner, FourCornerStrError};
use roxmltree::Node;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FourCornerError {
    #[error("(Four corner) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Four corner) Parsing: {0}, {1}")]
    Str(PosError, FourCornerStrError),
}

pub fn from(node: Node) -> Result<FourCorner, FourCornerError> {
    let text = shared::text(node)?;
    FourCorner::try_from(text).map_err(|err| FourCornerError::Str(PosError::from(node), err))
}

#[cfg(test)]
mod tests {
    use kanjidic_types::{FourCorner, Stroke};

    use super::from;
    use crate::test_shared::DOC;

    #[test]
    fn four_corner() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("q_code")
                    && node
                        .attribute("qc_type")
                        .map(|value| value.eq("four_corner"))
                        .unwrap_or(false)
            })
            .unwrap();
        let four_corner = from(node);
        assert_eq!(
            four_corner,
            Ok(FourCorner {
                top_left: Stroke::LineHorizontal,
                top_right: Stroke::Lid,
                bottom_left: Stroke::LineHorizontal,
                bottom_right: Stroke::Lid,
                fifth_corner: Some(Stroke::Box),
            })
        )
    }
}
