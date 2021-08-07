use serde::{Deserialize, Serialize};

/// An entry in the dictionary Daikanwajiten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Moro {
    /// The volume
    pub volume: Option<u8>,
    /// The page
    pub page: Option<u16>,
    /// The reference index
    pub index: MoroIndex,
}

/// The reference index
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MoroIndex {
    /// The item number
    pub index: u16,
    /// A letter that appears after the index
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
