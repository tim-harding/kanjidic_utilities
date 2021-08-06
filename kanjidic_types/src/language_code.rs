use serde::{Deserialize, Serialize};

/// A language used for translation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LanguageCode {
    /// English
    Eng,
    /// French
    Fra,
    /// Portuguese
    Por,
    /// Spanish
    Spa,
}
