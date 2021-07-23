use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;
use isolang::Language;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TranslationError {
    #[error("Node contains no text")]
    NoText,
    #[error("Unrecognized language code")]
    UnknownLanguage,
}

/// A translation of a kanji meaning into another language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Translation<'a> {
    /// The word in the target language.
    pub text: &'a str,

    /// The language being translated into.
    pub language: Language,
}

impl<'a, 'input> TryFrom<Node<'a, 'input>> for Translation<'a> {
    type Error = TranslationError;

    fn try_from(node: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let text = node.text().ok_or(TranslationError::NoText)?;
        let language = match node.attribute("m_lang") {
            Some(lang_text) => {
                Language::from_639_1(lang_text).ok_or(TranslationError::UnknownLanguage)?
            }
            None => Language::Eng,
        };
        Ok(Translation { text, language })
    }
}

#[cfg(test)]
mod tests {
    use super::Translation;
    use isolang::Language;
    use crate::test_shared::DOC;
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
