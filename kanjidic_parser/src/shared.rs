use roxmltree::Node;
use thiserror::Error;

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub type NomErr<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NomErrorReason {
    Incomplete,
    Error(nom::error::ErrorKind),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SharedError {
    #[error("Could not find a node with the given tag")]
    MissingTag(&'static str),
}

impl<'a> From<NomErr<'a>> for NomErrorReason {
    fn from(err: NomErr) -> Self {
        use nom::Err::*;

        match err {
            Incomplete(_) => NomErrorReason::Incomplete,
            Error(e) | Failure(e) => NomErrorReason::Error(e.code),
        }
    }
}

pub fn descendant<'a, 'input>(
    node: Node<'a, 'input>,
    tag: &'static str,
) -> Result<Node<'a, 'input>, SharedError> {
    node.descendants()
        .find(|child| child.has_tag_name(tag))
        .ok_or(SharedError::MissingTag(tag))
}
