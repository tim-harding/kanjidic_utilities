use kanjidic_types::Translations;
use roxmltree::Node;
use thiserror::Error;

use crate::shared::{self, SharedError};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TranslationError {
    #[error("Translation shared: {0}")]
    Shared(#[from] SharedError),
}

pub fn from(node: Node) -> Result<Translations, TranslationError> {
    let mut translations = Translations::default();
    let tmp: Result<Vec<(_, _)>, TranslationError> = node
        .children()
        .filter(|child| child.has_tag_name("meaning"))
        .map(|child| -> Result<(_, _), TranslationError> {
            let text = shared::text(child)?.to_owned();
            let language = child.attribute("m_lang").unwrap_or("en");
            Ok((language, text))
        })
        .collect();
    let tmp = tmp?;
    for translation in tmp {
        match translations.entry(translation.0.to_owned()) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(translation.1);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![translation.1.to_owned()]);
            }
        }
    }
    Ok(translations)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};

    use super::from;
    use crate::test_shared::DOC;

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
