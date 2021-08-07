use crate::{
    pos_error::PosError,
    shared::{self, IResult, NomErr, NomErrorReason, SharedError},
};
use kanjidic_types::{BusyPeople, Chapter};
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::char,
    combinator::{map, map_res, value},
    sequence::tuple,
};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum BusyPeopleError {
    #[error("(Busy people) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Busy people) Parsing: {0}, {1}")]
    Parse(PosError, BusyPeopleStrError),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum BusyPeopleStrError {
    #[error("(Busy people) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for BusyPeopleStrError {
    fn from(err: NomErr<'a>) -> Self {
        Self::Format(err.into())
    }
}

pub fn from(node: Node) -> Result<BusyPeople, BusyPeopleError> {
    let text = shared::text(node)?;
    from_str(text).map_err(|err| BusyPeopleError::Parse(PosError::from(node), err))
}

fn from_str(text: &str) -> Result<BusyPeople, BusyPeopleStrError> {
    let (_i, o) = parts(text)?;
    let (volume, _, chapter) = o;
    Ok(BusyPeople { volume, chapter })
}

fn parts(s: &str) -> IResult<(u8, char, Chapter)> {
    tuple((number, char('.'), chapter))(s)
}

fn chapter(s: &str) -> IResult<Chapter> {
    alt((
        value(Chapter::A, char('A')),
        map(number, |index| Chapter::Chapter { index }),
    ))(s)
}

fn number(s: &str) -> IResult<u8> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u8>()
    })(s)
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
                chapter: Chapter::Chapter { index: 14 },
            })
        )
    }
}
