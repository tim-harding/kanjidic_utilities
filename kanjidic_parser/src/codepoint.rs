use crate::shared::{self, attr, text_hex, SharedError};
use kanjidic_types::{Codepoint, Kuten, KutenParseError};
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CodepointError {
    #[error("(Codepoint) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Codepoint) Unrecognized encoding")]
    Encoding,
    #[error("(Codepoint) Kuten: {0}")]
    Kuten(#[from] KutenParseError),
}

pub fn from(node: Node) -> Result<Codepoint, CodepointError> {
    let text = shared::text(&node)?;
    let encoding = attr(&node, "cp_type")?;
    match encoding {
        "jis208" => Ok(Codepoint::Jis208(Kuten::try_from(text)?)),
        "jis212" => Ok(Codepoint::Jis212(Kuten::try_from(text)?)),
        "jis213" => Ok(Codepoint::Jis213(Kuten::try_from(text)?)),
        "ucs" => Ok(Codepoint::Unicode(text_hex(&node)?)),
        _ => Err(CodepointError::Encoding),
    }
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::Codepoint;

    #[test]
    fn codepoint() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("cp_value"))
            .unwrap();
        let code = from(node);
        assert_eq!(code, Ok(Codepoint::Unicode(20124)))
    }
}
