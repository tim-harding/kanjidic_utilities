use crate::{de_roo::DeRoo, kuten::{Kuten, PlanarKuten}, spahn_hadamitzky::SpahnHadamitzkyDescriptor};

/// Represents either of the following:
/// - A cross-reference to another kanji usually regarded as a variant
/// - An alternative indexing code for the current kanji
pub enum Variant {
    /// A coding in JIS X 0208
    Jis208(Kuten),
    
    /// A coding in JIS X 0212
    Jis212(Kuten),
    
    /// A coding in JIS X 0213
    Jis213(PlanarKuten),
    
    /// A unicode codepoint
    Unicode(char),
    
    /// An identification in the De Roo system
    DeRoo(DeRoo),
    
    /// Index in the NJECD system.
    Halpern(u16),
    
    /// The Kanji Dictionary kanji code.
    SpahnHadamitzky(SpahnHadamitzkyDescriptor),
    
    /// Index in the Modern Reader's Japanese-English dictionary.
    Nelson(u16),
    
    /// Index in Japanese Names by P.G. O'Neill.
    ONeill(u16),
}
