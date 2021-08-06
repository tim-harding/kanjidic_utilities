use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LanguageCodeError {
    #[error("Unknown language code")]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LanguageCode {
    Eng,
    Fra,
    Por,
    Spa,
}

impl TryFrom<&str> for LanguageCode {
    type Error = LanguageCodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "eng" => Ok(Self::Eng),
            "fra" => Ok(Self::Fra),
            "por" => Ok(Self::Por),
            "Spa" => Ok(Self::Spa),
            _ => Err(LanguageCodeError::Unknown),
        }
    }
}
