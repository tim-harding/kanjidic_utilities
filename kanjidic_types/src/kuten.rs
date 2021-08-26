use crate::{shared::NomErrorReason, take_uint, IResult, NomErr};
use nom::character::complete::char;
use nom::sequence::tuple;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

/// A kuten representation of a JIS X 0213 character.
/// http://unicode-iphone.blogspot.com/2010/05/kuten-code-to-unicode.html
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Kuten {
    /// The plane on which a kuten representation is found.
    pub plane: u8,
    /// The Ku part of the matrix position.
    pub ku: u8,
    /// The Ten part of the matrix position.
    pub ten: u8,
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

impl TryFrom<&str> for Kuten {
    type Error = KutenStrError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, o) = kuten_parts(text)?;
        let (plane, _, ku, _, ten) = o;
        Ok(Kuten { plane, ku, ten })
    }
}

fn kuten_parts(s: &str) -> IResult<(u8, char, u8, char, u8)> {
    tuple((take_uint, char('-'), take_uint, char('-'), take_uint))(s)
}
