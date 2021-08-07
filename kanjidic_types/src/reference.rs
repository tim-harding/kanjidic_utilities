use serde::{Deserialize, Serialize};

use crate::{BusyPeople, Moro, Oneill};

/// An index number into a particular kanji dictionary or reference book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum Reference {
    /// Modern Reader's Japanese-English Dictionary by Andrew Nelson
    NelsonClassic { index: u16 },
    /// The New Nelson Japanese-English Dictionary by John Haig
    NelsonNew { index: u16 },
    /// New Japanese-English Character Dictionary by Jack Halpern
    Njecd { index: u16 },
    /// Kodansha's Japanese-English Dictionary by Jack Halpern
    Kkd { index: u16 },
    /// Kanji Learners Dictionary by Jack Halpern
    Kkld { index: u16 },
    /// Kanji Learners Dictionary Second Edition by Jack Halpern
    Kkld2ed { index: u16 },
    /// Remembering the Kanji by James Heisig
    Heisig { index: u16 },
    /// Remembering the Kanji Sixth Edition by James Heisig
    Heisig6 { index: u16 },
    /// A New Dictionary of Kanji Usage
    Gakken { index: u16 },
    /// Japanese Names by P.G. O'Neill
    OneillNames(Oneill),
    /// Essential Kanji by P.G. O'Neill
    OneillKk { index: u16 },
    /// Daikanwajiten by Morohashi
    Moro(Moro),
    /// A Guide to Remembering Japanese Characters by Kenneth G. Henshall
    Henshall { index: u16 },
    /// Kanji and Kana by Spahn and Hadamitzky
    ShKk { index: u16 },
    /// Kanji and Kana 2011 edition by Spahn and Hadamitzky
    ShKk2 { index: u16 },
    /// A Guide to Reading and Writing Japanese by Florence Sakade
    Sakade { index: u16 },
    /// Japanese Kanji Flashcards by Tomoko Okazaki
    Jfcards { index: u16 },
    /// A Guide to Reading and Writing Japanese by Henshall
    Henshall3 { index: u16 },
    /// Tuttle Kanji Cards by Alexander Kask
    TuttleCards { index: u16 },
    /// The Kanji Way to Japanese Language Power by Dale Crowley
    Crowley { index: u16 },
    /// Kanji in Context by Nishiguchi and Kono
    KanjiInContext { index: u16 },
    /// Japanese for Busy People
    BusyPeople(BusyPeople),
    /// The Kodansha Compact Study Guide
    KodanshaCompact { index: u16 },
    /// Les Kanjis dans la tete by Yves Maniette
    Maniette { index: u16 },
}
