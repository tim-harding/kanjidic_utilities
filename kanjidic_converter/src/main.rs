use kanjidic_parser::stuff;
use thiserror::Error;

#[derive(Debug, Error)]
enum KdcError {
    #[error("Error reading or writing file")]
    Io(#[from] std::io::Error),
}

fn main() -> Result<(), KdcError> {
    let xml = std::fs::read_to_string("./assets/kanjidic.xml")?;
    stuff(&xml);
    Ok(())
}