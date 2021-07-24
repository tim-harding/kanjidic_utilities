use isolang::Language;
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TranslationError {
    #[error("Error from shared utilities: {0}")]
    Shared(#[from] SharedError),
    #[error("Unrecognized language code: {0}")]
    UnknownLanguage(PosError),
}

// Todo: Identify suffixes and prefixes
/// A translation of a kanji meaning into another language.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Translation<'a> {
    /// The word in the target language.
    pub text: &'a str,
    /// The language being translated into.
    pub language: Language,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Translation<'a> {
    type Error = TranslationError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = shared::text(node)?;
        let language = match node.attribute("m_lang") {
            Some(lang_text) => Language::from_639_1(lang_text)
                .ok_or(TranslationError::UnknownLanguage(PosError::from(node)))?,
            None => Language::Eng,
        };
        Ok(Translation { text, language })
    }
}

#[cfg(test)]
mod tests {
    use super::Translation;
    use crate::test_shared::DOC;
    use isolang::Language;
    use std::convert::TryFrom;

    #[test]
    fn translation() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("meaning"))
            .unwrap();
        let translation = Translation::try_from(node);
        assert_eq!(
            translation,
            Ok(Translation {
                text: "Asia",
                language: Language::Eng,
            })
        )
    }
}
