/// Kanji code from the SKIP system of indexing.
/// http://www.edrdg.org/wwwjdic/SKIP.html
pub enum Skip {
    /// Pattern 1, the kanji can be divided into left and right parts.
    Horizontal(SkipHorizontal),

    /// Pattern 2, the kanji can be divided into top and bottom parts.
    Vertical(SkipVertical),

    /// Pattern 3, the kanji can be divided by an enclosure element.
    Enclosure(SkipEnclosure),

    /// Pattern 4, the cannot be classified by any of the above patterns.
    Solid(SkipSolid),
}

/// Left and right parts of the kanji.
pub struct SkipHorizontal {
    /// Number of strokes in the left part.
    pub left: u8,

    /// Number of strokes in the right part.
    pub right: u8,
}

/// Top and bottom parts of the kanji.
pub struct SkipVertical {
    /// Number of strokes in the top part.
    pub top: u8,

    /// Number of strokes in the bottom part.
    pub bottom: u8,
}

/// Interior and exterior parts of the kanji.
pub struct SkipEnclosure {
    /// Number of strokes in the exterior part.
    pub exterior: u8,

    /// Number of strokes in the interior part.
    pub interior: u8,
}

/// Classification for kanji that don't fit another pattern.
pub struct SkipSolid {
    /// The total number of strokes in the kanji.
    pub total_stroke_count: u8,

    /// The subpattern that defines the kanji.
    pub solid_subpattern: SolidSubpattern,
}

/// An identifying characteristic of the kanji.
pub enum SolidSubpattern {
    /// Contains a top line.
    TopLine = 1,

    /// Contains a bottom line.
    BottomLine,

    /// Contains a through line.
    ThroughLine,

    /// Does not contain any of the above.
    Other,
}
