use crate::Kuten;
use serde::{Deserialize, Serialize};

/// The code of a kanji in a given character set standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum Codepoint {
    /// Encoding in JIS X 0208-1997
    Jis208(Kuten),
    /// Encoding in JIS X 0212-1990
    Jis212(Kuten),
    /// Encoding in JIS X 0213-2000
    Jis213(Kuten),
    /// Unicode character
    Unicode(u32),
}
