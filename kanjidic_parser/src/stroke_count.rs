/// The number of strokes in a kanji.
pub struct StrokeCount {
    /// The accepted number of strokes.
    pub accepted: u8,
    
    /// Possible miscounts of the stroke count.
    pub miscounts: Vec<u8>,
}
