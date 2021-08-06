use serde::{Deserialize, Serialize};

/// The date the file was created
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DateOfCreation {
    /// Year of creation
    pub year: u16,
    /// Month of creation
    pub month: u8,
    /// Day of creation
    pub day: u8,
}
