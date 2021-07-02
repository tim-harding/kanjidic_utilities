mod database_version;
mod shared;
mod header;
mod date_of_creation;

#[cfg(test)]
mod test_shared;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use database_version::DatabaseVersionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdpError {
    #[error("Error parsing XML file")]
    Xml(#[from] roxmltree::Error),
    #[error("Error parsing database version")]
    DatabaseVersion(#[from] DatabaseVersionError),
}

pub fn parse(xml: &str) -> Result<(), KdpError> {
    let _doc = roxmltree::Document::parse(xml)?;
    Ok(())
}