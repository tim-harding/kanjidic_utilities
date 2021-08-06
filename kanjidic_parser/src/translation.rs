use kanjidic_types::{LanguageCode, Translation};
use roxmltree::Node;
use thiserror::Error;

use crate::{
    language_code,
    pos_error::PosError,
    shared::{self, SharedError},
    LanguageCodeError,
};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TranslationError {
    #[error("Translation shared: {0}")]
    Shared(#[from] SharedError),
    #[error("Translation unknown language code: {0}")]
    UnknownLanguage(PosError, LanguageCodeError),
}

pub fn from(node: Node) -> Result<Translation, TranslationError> {
    let text = shared::text(node)?.into();
    let language = match node.attribute("m_lang") {
        Some(lang_text) => language_code::from(lang_text)
            .map_err(|err| TranslationError::UnknownLanguage(PosError::from(node), err))?,
        None => LanguageCode::Eng,
    };
    Ok(Translation { text, language })
}

#[cfg(test)]
mod tests {
    use kanjidic_types::{LanguageCode, Translation};

    use super::from;
    use crate::test_shared::DOC;

    #[test]
    fn translation() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("meaning"))
            .unwrap();
        let translation = from(node);
        assert_eq!(
            translation,
            Ok(Translation {
                text: "Asia".into(),
                language: LanguageCode::Eng,
            })
        )
    }
}
