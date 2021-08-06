use serde::{Deserialize, Serialize};

/// An index into the Japanese Names reference book
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Oneill {
    /// The reference number
    pub number: u16,
    /// A reference's suffix
    pub suffix: OneillSuffix,
}

/// The suffix for a Japanese Names reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OneillSuffix {
    /// No suffix
    None,
    /// 'A' suffix
    A,
}
