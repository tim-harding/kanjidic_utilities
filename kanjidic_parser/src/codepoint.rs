use crate::kuten::{Kuten, PlanarKuten};

/// The code of a kanji in a given character set standard.
pub enum Codepoint {
    /// Encoding in JIS X 0208-1997
    Jis208(Kuten),
    
    /// Encoding in JIS X 0212-1990
    Jis212(Kuten),
    
    /// Encoding in JIS X 0213-2000
    Jis213(PlanarKuten),
}
