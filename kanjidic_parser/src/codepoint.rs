use std::convert::TryFrom;
use thiserror::Error;
use roxmltree::Node;
use crate::kuten::{Kuten, PlanarKuten};

/// The code of a kanji in a given character set standard.
pub enum Codepoint {
    /// Encoding in JIS X 0208-1997
    Jis208(Kuten),
    
    /// Encoding in JIS X 0212-1990
    Jis212(Kuten),
    
    /// Encoding in JIS X 0213-2000
    Jis213(PlanarKuten),
    
    /// Encoding in Unicode
    Ucs(char),
}

#[derive(Debug, Error)]
pub enum CodepointError {
    #[error("Codepoint type not recognized")]
    CpType,
    #[error("No text")]
    NoText,
}


impl<'a, 'input> TryFrom<Node<'a, 'input>> for Codepoint {
    type Error = CodepointError;
    fn try_from(node: Node) -> Result<Codepoint, CodepointError> {
        let text = node.text().ok_or(CodepointError::NoText)?;
        match node.attribute("cp_type") {
            "jis208" => Ok(Codepoint::Jis208(text.into())),
            "jis212" => Ok(Codepoint::Jis212(text.into())),
            "jis213" => Ok(Codepoint::Jis213(text.into())),
            "ucs" => Ok(Codepoint::Ucs),
            _ => Err(CodepointError::CpType),
        }
    }
}