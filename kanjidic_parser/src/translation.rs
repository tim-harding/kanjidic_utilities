use kanjidic_types::Translations;
use roxmltree::Node;
use thiserror::Error;

use crate::shared::{self, SharedError};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TranslationError {
    #[error("Translation shared: {0}")]
    Shared(#[from] SharedError),
}

pub fn add_meaning(
    translations: &mut Translations,
    meaning: &Node,
) -> Result<(), TranslationError> {
    let text = shared::text(meaning)?.to_owned();
    let language = meaning.attribute("m_lang").unwrap_or("en").to_owned();
    match translations.entry(language) {
        std::collections::hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().push(text);
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(vec![text]);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};
    use kanjidic_types::Translations;
    use roxmltree::Node;
    use crate::{TranslationError, add_meaning, test_shared::DOC};

    // Just keeping this around for now for the test
    pub fn from(node: Node) -> Result<Translations, TranslationError> {
        let mut translations = Translations::default();
        for child in node
            .children()
            .filter(|child| child.has_tag_name("meaning"))
        {
            add_meaning(&mut translations, &child)?;
        }
        Ok(translations)
    }

    #[test]
    fn translation() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("rmgroup"))
            .unwrap();
        let translation = from(node);
        assert_eq!(
            translation,
            Ok(HashMap::from_iter([
                (
                    "en".to_owned(),
                    vec![
                        "Asia".to_owned(),
                        "rank next".to_owned(),
                        "come after".to_owned(),
                        "-ous".to_owned(),
                    ]
                ),
                (
                    "fr".to_owned(),
                    vec![
                        "Asie".to_owned(),
                        "suivant".to_owned(),
                        "sub-".to_owned(),
                        "sous-".to_owned(),
                    ]
                ),
                (
                    "pt".to_owned(),
                    vec![
                        "Ásia".to_owned(),
                        "próxima".to_owned(),
                        "o que vem depois".to_owned(),
                        "-ous".to_owned(),
                    ]
                ),
                (
                    "es".to_owned(),
                    vec![
                        "pref. para indicar".to_owned(),
                        "venir después de".to_owned(),
                        "Asia".to_owned(),
                    ]
                )
            ]))
        )
    }
}
