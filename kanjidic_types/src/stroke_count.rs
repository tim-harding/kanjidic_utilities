use serde::{Deserialize, Serialize};

/// The number of strokes in a kanji.
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StrokeCount {
    /// The accepted number of strokes.
    pub accepted: u8,
    /// Possible miscounts of the stroke count.
    pub miscounts: Vec<u8>,
}
