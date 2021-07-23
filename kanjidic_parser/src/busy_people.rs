/// A location in Japanese for Busy People.
pub struct BusyPeople {
    /// The volume
    volume: u8,

    /// The chapter
    chapter: Chapter,
}

/// Either the chapter number or chapter A in Japanese for Busy People.
pub enum Chapter {
    /// A chapter number.
    Chapter(u8),

    /// Some of the chapter are called "A",
    /// but it isn't specified anywhere what that means.
    A,
}