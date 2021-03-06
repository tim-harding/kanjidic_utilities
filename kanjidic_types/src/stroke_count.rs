use serde::{Deserialize, Serialize};

/// The number of strokes in a kanji.
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrokeCount {
    /// The accepted number of strokes.
    pub accepted: u8,
    /// Possible miscounts of the stroke count.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub miscounts: Vec<u8>,
}
