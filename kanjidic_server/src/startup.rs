use std::collections::hash_map::Entry;

use crate::cache::{Cache, KanjiCache, Radk, RadkCache, TranslationCache};
use kanjidic_types::Character;
use rocket::{
    fairing,
    tokio::{fs::read_to_string, io},
    Build, Rocket,
};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum InitError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

pub async fn init_cache(rocket: Rocket<Build>, kanji_path: String) -> fairing::Result {
    let (kanji, translations) = match get_kanji_data(kanji_path).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            return Err(rocket);
        }
    };
    let radk = get_radk_data();
    let cache = Cache {
        kanji,
        translations,
        radk,
    };
    Ok(rocket.manage(cache))
}

fn get_radk_data() -> RadkCache {
    kradical_static::MEMBERSHIPS
        .iter()
        .map(|membership| {
            let radk = Radk {
                radical: membership.radical,
                stroke: membership.strokes,
                kanji: membership.kanji.iter().cloned().collect(),
            };
            (radk.radical, radk)
        })
        .collect()
}

async fn get_kanji_data(kanji_path: String) -> Result<(KanjiCache, TranslationCache), InitError> {
    let mut kanji = KanjiCache::default();
    let mut translations = TranslationCache::default();
    let file_contents = read_to_string(kanji_path).await?;
    let kanji_json: Vec<Character> = serde_json::from_str(&file_contents)?;
    for character in kanji_json {
        let literal = character.literal;
        for language in character.translations.values() {
            for translation in language.iter() {
                for part in translation.split(' ') {
                    match translations.entry(part.to_owned()) {
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().push(literal);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(vec![literal]);
                        }
                    }
                }
            }
        }
        kanji.insert(literal, character);
    }
    Ok((kanji, translations))
}
