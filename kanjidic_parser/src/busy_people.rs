use crate::{
    pos_error::PosError,
    shared::{text, SharedError},
};
use kanjidic_types::{busy_people, BusyPeople};
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("(Busy people) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Busy people) Parsing: {0}, {1}")]
    Parse(PosError, busy_people::ParseError),
}

pub fn from(node: Node) -> Result<BusyPeople, Error> {
    let text = text(&node)?;
    BusyPeople::try_from(text).map_err(|err| Error::Parse(PosError::from(&node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::BusyPeople;

    #[test]
    fn busy_people() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("dic_ref")
                    && node
                        .attribute("dr_type")
                        .map(|value| value.eq("busy_people"))
                        .unwrap_or(false)
            })
            .unwrap();
        let busy_people = from(node);
        assert_eq!(
            busy_people,
            Ok(BusyPeople {
                volume: 3,
                chapter: Some(14),
            })
        )
    }
}
