use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};
use kanjidic_types::{four_corner, FourCorner};
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("(Four corner) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Four corner) Parsing: {0}, {1}")]
    Str(PosError, four_corner::ParseError),
}

pub fn from(node: Node) -> Result<FourCorner, Error> {
    let text = shared::text(&node)?;
    FourCorner::try_from(text).map_err(|err| Error::Str(PosError::from(&node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{four_corner::Stroke, FourCorner};

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
