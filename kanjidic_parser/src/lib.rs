use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdpError {
    #[error("Error parsing XML file")]
    Parse(#[from] roxmltree::Error),
}

pub fn parse(xml: &str) -> Result<(), KdpError> {
    let doc = roxmltree::Document::parse(xml)?;
    for descendant in doc.descendants() {
        println!("{:?}", descendant.tag_name())
    }
    Ok(())
}

struct Header {
    file_version: u8,
    database_version: DatabaseVersion,
    date_of_creation: DateOfCreation,
}

struct DateOfCreation {
    year: u16,
    month: u8,
    date: u8,
}

struct DatabaseVersion {
    year: u16,
    version: u16,
}