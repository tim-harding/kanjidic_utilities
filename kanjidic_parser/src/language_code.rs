use kanjidic_types::LanguageCode;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LanguageCodeError {
    #[error("Unknown language code")]
    Unknown,
}

pub fn from(value: &str) -> Result<LanguageCode, LanguageCodeError> {
    match value {
        "eng" => Ok(LanguageCode::Eng),
        "fra" => Ok(LanguageCode::Fra),
        "por" => Ok(LanguageCode::Por),
        "spa" => Ok(LanguageCode::Spa),
        "en" => Ok(LanguageCode::Eng),
        "fr" => Ok(LanguageCode::Fra),
        "pt" => Ok(LanguageCode::Por),
        "es" => Ok(LanguageCode::Spa),
        _ => Err(LanguageCodeError::Unknown),
    }
}
