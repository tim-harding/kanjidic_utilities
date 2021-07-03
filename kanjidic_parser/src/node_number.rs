use std::str::FromStr;

use nom::{bytes::complete::take_while1, combinator::map_res};
use roxmltree::Node;
use thiserror::Error;

use crate::shared::{IResult, NomErr};

/// An error while parsing the file version.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum NodeNumberError {
    /// The tag contained no text.
    #[error("No file version text")]
    NoText,

    /// The tag text was not digits.
    #[error("Failed to parse version")]
    Format,

    /// The tag text could not be parsed as a number.
    #[error("Could not convert text to a number")]
    Convert,
}

pub fn node_number<T: FromStr>(node: Node) -> Result<T, NodeNumberError> {
    let file_version_text = node.text().ok_or(NodeNumberError::NoText)?;
    Ok(digits(file_version_text)
        .map(|(_, v)| v)
        .map_err(|_e: NomErr| NodeNumberError::Format)?)
}

fn digits<T: FromStr>(s: &str) -> IResult<T> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), map_file_version)(s)
}

fn map_file_version<T: FromStr>(n: &str) -> Result<T, NodeNumberError> {
    let version: T = n.parse().map_err(|_| NodeNumberError::Convert)?;
    Ok(version)
}
