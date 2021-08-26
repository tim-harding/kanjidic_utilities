use std::convert::TryFrom;

use crate::{
    pos_error::PosError,
    shared::{text, SharedError},
};
use kanjidic_types::{Oneill, OneillStrError};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OneillError {
    #[error("(Oneill) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Oneill) Parsing: {0}, {1}")]
    Parse(PosError, OneillStrError),
}

pub fn from(node: Node) -> Result<Oneill, OneillError> {
    Oneill::try_from(text(node)?).map_err(|err| OneillError::Parse(PosError::from(node), err))
}
