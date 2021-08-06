use roxmltree::Node;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
    LanguageCode, LanguageCodeError,
};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TranslationError {
    #[error("Translation shared: {0}")]
    Shared(#[from] SharedError),
    #[error("Translation unknown language code: {0}")]
    UnknownLanguage(PosError, LanguageCodeError),
}

// Todo: Identify suffixes and prefixes
/// A translation of a kanji meaning into another language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Translation {
    /// The word in the target language.
    pub text: String,
    /// The language being translated into.
    pub language: LanguageCode,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Translation {
    type Error = TranslationError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let text = shared::text(node)?.into();
        let language = match node.attribute("m_lang") {
            Some(lang_text) => LanguageCode::try_from(lang_text)
                .map_err(|err| TranslationError::UnknownLanguage(PosError::from(node), err))?,
            None => LanguageCode::Eng,
        };
        Ok(Translation { text, language })
    }
}

#[cfg(test)]
mod tests {
    use super::Translation;
    use crate::{test_shared::DOC, LanguageCode};
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
                text: "Asia".into(),
                language: LanguageCode::Eng,
            })
        )
    }
}
