use crate::kuten::{Kuten, KutenError, KutenStrError};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CodepointError {
    #[error("Node contains no text")]
    NoText,
    #[error("Unrecognized encoding")]
    Encoding,
    #[error("Could not parse hexadecimal")]
    Hex,
    #[error("Could not parse kuten")]
    Kuten(#[from] KutenStrError),
}

/// The code of a kanji in a given character set standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Codepoint {
    type Error = CodepointError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = node.text().ok_or(CodepointError::NoText)?;
        let encoding = node.attribute("cp_type").ok_or(CodepointError::Encoding)?;
        match encoding {
            "jis208" => Ok(Codepoint::Jis208(Kuten::try_from(text)?)),
            "jis212" => Ok(Codepoint::Jis212(Kuten::try_from(text)?)),
            "jis213" => Ok(Codepoint::Jis213(Kuten::try_from(text)?)),
            "ucs" => {
                let code = u32::from_str_radix(&text, 16).map_err(|_| CodepointError::Hex)?;
                Ok(Codepoint::Unicode(code))
            }
            _ => Err(CodepointError::Encoding),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Codepoint;
    use crate::test_shared::DOC;
    use std::convert::TryFrom;

    #[test]
    fn codepoint() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("cp_value"))
            .unwrap();
        let code = Codepoint::try_from(node);
        assert_eq!(code, Ok(Codepoint::Unicode(20124)))
    }
}
