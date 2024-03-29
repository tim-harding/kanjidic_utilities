use clap::Parser;
use kanjidic_parser::kanjidic::{self, Kanjidic};
use std::{convert::TryFrom, fs};
use thiserror::Error;

#[derive(Debug, Error)]
enum KdcError {
    #[error("Error reading or writing file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error parsing file: {0}")]
    Parse(#[from] kanjidic::Error),
    #[error("Could not skip DTD section")]
    DtdSkip,
    #[error("Failed to get back UTF8 after skipping DTD: {0}")]
    DtdSkipUtf8(#[from] std::str::Utf8Error),
    #[error("Error from json serialization: {0}")]
    Json(Box<dyn std::error::Error>),
}

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    output: String,
    #[clap(short, long)]
    header: bool,
}

fn main() -> Result<(), KdcError> {
    let opts: Opts = Opts::parse();
    let xml = std::fs::read_to_string(opts.input)?;
    let start = xml.find("<kanjidic2>").ok_or(KdcError::DtdSkip)?;
    let skipped = std::str::from_utf8(&xml.as_bytes()[start..])?;
    let kanjidic = Kanjidic::try_from(skipped)?;
    let json = if opts.header {
        serde_json::to_string_pretty(&kanjidic)
    } else {
        serde_json::to_string_pretty(&kanjidic.characters)
    }
    .map_err(|err| KdcError::Json(err.into()))?;
    fs::write(opts.output, json)?;
    Ok(())
}
