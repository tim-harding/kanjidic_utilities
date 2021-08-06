use serde::{Deserialize, Serialize};

use crate::{DatabaseVersion, DateOfCreation};

/// Contains identification information about the version of the file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Header {
    /// Denotes the version of the database structure.
    pub file_version: u8,
    /// The database version.
    pub database_version: DatabaseVersion,
    /// The date on which the database was created.
    pub date_of_creation: DateOfCreation,
}
