use crate::pos_error::PosError;
use roxmltree::Node;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SharedError {
    #[error("(Shared) No node with the given tag: {0}, attribute '{1}'")]
    MissingChild(PosError, &'static str),
    #[error("(Shared) Node contains no text")]
    NoText(PosError),
    #[error("(Shared) Could not parse text as a uint")]
    TextUint(PosError),
    #[error("(Shared) Could not parse attribute as a uint")]
    AttrUint(PosError),
    #[error("(Shared) Node missing attribute: {0}, attribute '{1}'")]
    MissingAttribute(PosError, &'static str),
    #[error("(Shared) Could not parse hexadecimal")]
    Hex(PosError),
}

pub fn children<'a, 'input, T, E, F>(
    node: &Node<'a, 'input>,
    tag: &'static str,
    cb: F,
) -> Result<Vec<T>, E>
where
    E: std::error::Error,
    F: Fn(Node<'a, 'input>) -> Result<T, E>,
{
    node.children()
        .filter(|child| child.has_tag_name(tag))
        .map(cb)
        .collect()
}

pub fn text_uint<T: FromStr>(node: &Node) -> Result<T, SharedError> {
    text(node)?
        .parse::<T>()
        .map_err(|_| SharedError::TextUint(PosError::from(node)))
}

pub fn text_hex(node: &Node) -> Result<u32, SharedError> {
    let text = text(node)?;
    u32::from_str_radix(text, 16).map_err(|_| SharedError::Hex(PosError::from(node)))
}

pub fn text<'a>(node: &Node<'a, '_>) -> Result<&'a str, SharedError> {
    node.text()
        .ok_or_else(|| SharedError::NoText(PosError::from(node)))
}

pub fn attr<'a>(
    node: &Node<'a, '_>,
    attribute: &'static str,
) -> Result<&'a str, SharedError> {
    node.attribute(attribute)
        .ok_or_else(|| SharedError::MissingAttribute(PosError::from(node), attribute))
}

pub fn attr_uint<T: FromStr>(
    node: &Node,
    attribute: &'static str,
) -> Result<Option<T>, SharedError> {
    match node.attribute(attribute) {
        Some(text) => {
            let parsed: T = text
                .parse()
                .map_err(|_| SharedError::AttrUint(PosError::from(node)))?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}
