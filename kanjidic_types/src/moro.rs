use serde::{Deserialize, Serialize};

/// An entry in the dictionary Daikanwajiten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Moro {
    /// The volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<u8>,
    /// The page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u16>,
    /// The item number
    pub index: u16,
    /// A letter that appears after the index
    #[serde(skip_serializing_if = "MoroSuffix::is_none")]
    pub suffix: MoroSuffix,
}

/// A letter that appears at the end of the index
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MoroSuffix {
    /// No suffix
    None,
    /// P suffix
    P,
    /// X suffix
    X,
    /// PX suffix
    PX,
}

impl MoroSuffix {
    pub fn is_none(&self) -> bool {
        *self == Self::None
    }
}