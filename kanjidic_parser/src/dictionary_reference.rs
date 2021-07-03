/// An index number into a particular kanji dictionary or reference book.
pub enum DictionaryReference {
    /// Modern Reader's Japanese-English Dictionary by Andrew Nelson
    NelsonClassic(u16),
    
    /// The New Nelson Japanese-English Dictionary by John Haig
    NelsonNew(u16),
    
    /// New Japanese-English Character Dictionary by Jack Halpern
    Njecd(u16),
    
    /// Kodansha's Japanese-English Dictionary by Jack Halpern
    Kkd(u16),
    
    /// Kanji Learners Dictionary by Jack Halpern
    Kkld(u16),
    
    /// Kanji Learners Dictionary Second Edition by Jack Halpern
    Kkld2ed(u16),
    
    /// Remembering the Kanji by James Heisig
    Heisig(u16),
    
    /// Remembering the Kanji Sixth Edition by James Heisig
    Heisig6(u16),
    
    /// A New Dictionary of Kanji Usage
    Gakken(u16),
    
    /// Japanese Names by P.G. O'Neill
    OneillNames(u16),
    
    /// Essential Kanji by P.G. O'Neill
    OneillKk(u16),
    
    /// Daikanwajiten by Morohashi
    Moro(Moro),
    
    /// A Guide to Remembering Japanese Characters by Kenneth G. Henshall
    Henshall(u16),
    
    /// Kanji and Kana by Spahn and Hadamitzky
    ShKk(u16),
    
    /// Kanji and Kana 2011 edition by Spahn and Hadamitzky
    ShKk2(u16),
    
    /// A Guide to Reading and Writing Japanese by Florence Sakade
    Sakade(u16),
    
    /// Japanese Kanji Flashcards by Tomoko Okazaki
    Jfcards(u16),
    
    /// The Kanji Way to Japanese Language Power by Dale Crowley
    Crowley(u16),
    
    /// Kanji in Context by Nishiguchi and Kono
    KanjiInContext(u16),
    
    /// Japanese for Busy People
    BusyPeople(BusyPeople),
    
    /// The Kodansha Compact Study Guide
    KodanshaCompact(u16),
    
    /// Les Kanjis dans la tete by Yves Maniette
    Maniette(u16),
}

/// A location in Japanese for Busy People.
pub struct BusyPeople {
    /// The volume
    volume: u8,
    
    /// The chapter
    chapter: Chapter,
}

/// Either the chapter number of chapter A in Japanese for Busy People.
pub enum Chapter {
    /// A chapter number.
    Chapter(u8),
    
    /// Some of the chapter are called "A", 
    /// but it isn't specified anywhere what that means.
    A,
}

/// An entry in the dictionary Daikanwajiten.
pub struct Moro {
    /// The volume
    pub volume: Option<u8>,
    
    /// The page
    pub page: Option<u16>,
    
    /// The item number
    pub item: u16,
}
