use std::convert::TryFrom;

use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};
use kanjidic_types::{BusyPeople, BusyPeopleParseError};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum BusyPeopleError {
    #[error("(Busy people) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Busy people) Parsing: {0}, {1}")]
    Parse(PosError, BusyPeopleParseError),
}

pub fn from(node: Node) -> Result<BusyPeople, BusyPeopleError> {
    let text = shared::text(node)?;
    BusyPeople::try_from(text).map_err(|err| BusyPeopleError::Parse(PosError::from(node), err))
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{BusyPeople, Chapter};

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
                chapter: Chapter::Numbered(14),
            })
        )
    }
}
