use kanjidic_parser::{KanjidicDocument};
use thiserror::Error;

#[derive(Debug, Error)]
enum KdcError {
    #[error("Error reading or writing file")]
    Io(#[from] std::io::Error),
    #[error("Error parsing file")]
    Parse(#[from] kanjidic_parser::KanjidicError),
    #[error("Could not skip DTD section")]
    DtdSkip,
    #[error("Failed to get back UTF8 after skipping DTD")]
    DtdSkipUtf8(#[from] std::str::Utf8Error),
}

fn main() -> Result<(), KdcError> {
    let xml = std::fs::read_to_string("./assets/kanjidic2.xml")?;
    let start = xml.find("<kanjidic2>").ok_or(KdcError::DtdSkip)?;
    let skipped = std::str::from_utf8(&xml.as_bytes()[start..])?;
    let doc = KanjidicDocument::new(&skipped)?;
    let _dic = doc.kanjidic();
    Ok(())
}
