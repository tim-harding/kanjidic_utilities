use crate::shared::{IResult, NomErr};
use nom::{bytes::complete::take_while1, combinator::map_res};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FileVersionError {
    #[error("No file version text")]
    NoFileVersionText,
    #[error("Failed to parse version")]
    Parse,
    #[error("Could not parse version as a number")]
    Number,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileVersion(pub u8);

impl<'a, 'input> TryFrom<Node<'a, 'input>> for FileVersion {
    type Error = FileVersionError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let file_version_text = node.text().ok_or(FileVersionError::NoFileVersionText)?;
        Ok(Self(
            parse_file_version(file_version_text)
                .map(|(_, v)| v)
                .map_err(|_e: NomErr| FileVersionError::Parse)?,
        ))
    }
}

fn parse_file_version(s: &str) -> IResult<u8> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), map_file_version)(s)
}

fn map_file_version(n: &str) -> Result<u8, FileVersionError> {
    let version: u8 = n.parse().map_err(|_| FileVersionError::Number)?;
    Ok(version)
}
