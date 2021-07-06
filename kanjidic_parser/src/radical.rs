use std::convert::TryFrom;
use crate::kangxi::KangXi;
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum RadicalError {
    #[error("Node contains no text")]
    NoText,
    #[error("The radical is not in a valid range")]
    OutOfRange,
    #[error("Could not parse text content as a number")]
    Number,
    #[error("Not a recognized radical kind")]
    Kind,
}

/// A kanji classification based on its radical.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Radical {
    /// Based on the KangXi Zidian system.
    /// Referenced from the Shibano JIS Kanwa Jiten.
    Classical(KangXi),

    /// As used in the classic Modern Japanese-English Character Dictionary.
    Nelson(KangXi),
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Radical {
    type Error = RadicalError;
    
    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        
    }
}