use serde::{Deserialize, Serialize};

/// The version of the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DatabaseVersion {
    /// The year of release.
    pub year: u16,
    /// The version that came out in the given year,
    /// with the counter being reset annually.
    pub version: u16,
}
