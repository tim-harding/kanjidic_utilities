use serde::{Deserialize, Serialize};

/// A location in Japanese for Busy People.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BusyPeople {
    /// The volume
    pub volume: u8,
    /// The chapter
    pub chapter: Chapter,
}

/// Either the chapter number or chapter A in Japanese for Busy People.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum Chapter {
    /// A chapter number.
    Numbered(u8),
    /// Some of the chapter are called "A",
    /// but it isn't specified anywhere what that means.
    A,
}
