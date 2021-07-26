use roxmltree::Node;
use thiserror::Error;
use kanjidic_types::{Language, Translation};

use crate::{
    pos_error::PosError,
    shared::{self, SharedError},
};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TranslationError {
    #[error("Translation shared: {0}")]
    Shared(#[from] SharedError),
    #[error("Translation unknown language code: {0}")]
    UnknownLanguage(PosError),
}

fn parse_translation(node: Node) -> Result<Translation, TranslationError> {
    let text = shared::text(node)?;
    let language = match node.attribute("m_lang") {
        Some(lang_text) => Language::from_639_1(lang_text)
            .ok_or(TranslationError::UnknownLanguage(PosError::from(node)))?,
        None => Language::Eng,
    };
    Ok(Translation { text, language })
}

#[cfg(test)]
mod tests {
    use kanjidic_types::Language;

    use super::Translation;
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
