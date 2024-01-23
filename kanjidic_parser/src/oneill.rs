use crate::{
    pos_error::PosError,
    shared::{text, SharedError},
};
use kanjidic_types::{oneill, Oneill};
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("(Oneill) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Oneill) Parsing: {0}, {1}")]
    Parse(PosError, oneill::ParseError),
}

pub fn from(node: Node) -> Result<Oneill, Error> {
    Oneill::try_from(text(&node)?).map_err(|err| Error::Parse(PosError::from(&node), err))
}
