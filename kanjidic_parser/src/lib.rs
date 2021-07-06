mod character;
mod codepoint;
mod database_version;
mod date_of_creation;
mod de_roo;
mod dictionary_reference;
mod four_corner;
mod grade;
mod header;
mod kunyomi;
mod kuten;
mod meaning;
mod node_number;
mod pin_yin;
mod query_code;
mod radical;
mod reading;
mod shared;
mod skip;
mod spahn_hadamitzky;
mod stroke_count;
mod translation;
mod variant;
mod kangxi;

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
