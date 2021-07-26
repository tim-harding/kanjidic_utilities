use serde::{Serialize, Deserialize};

mod kangxi;
pub use kangxi::KangXi;

pub use isolang::Language;
pub use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

/// Represents either of the following:
/// - A cross-reference to another kanji usually regarded as a variant
/// - An alternative indexing code for the current kanji
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Variant {
    /// A coding in JIS X 0208
    Jis208(Kuten),
    /// A coding in JIS X 0212
    Jis212(Kuten),
    /// A coding in JIS X 0213
    Jis213(Kuten),
    /// A unicode codepoint
    Unicode(u32),
    /// An identification in the De Roo system
    DeRoo(DeRoo),
    /// Index in the NJECD system.
    Halpern(u16),
    /// The Kanji Dictionary kanji code.
    Sh(ShDesc),
    /// Index in the Modern Reader's Japanese-English dictionary.
    Nelson(u16),
    /// Index in Japanese Names by P.G. O'Neill.
    ONeill(Oneill),
}

/// The number of strokes in a kanji.
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StrokeCount {
    /// The accepted number of strokes.
    pub accepted: u8,
    /// Possible miscounts of the stroke count.
    pub miscounts: Vec<u8>,
}

/// An index into the Japanese Names reference book
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Oneill {
    /// The reference number
    pub number: u16,
    /// A reference's suffix
    pub suffix: OneillSuffix,
}

/// The suffix for a Japanese Names reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OneillSuffix {
    /// No suffix
    None,
    /// 'A' suffix
    A,
}

// Todo: Identify suffixes and prefixes
/// A translation of a kanji meaning into another language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Translation {
    /// The word in the target language.
    pub text: String,
    /// The language being translated into.
    pub language: Language,
}

/// Descriptor code for The Kanji Dictionary.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ShDesc {
    /// Number of strokes in the identifying radical.
    pub radical_strokes: u8,
    /// The letter for the radical in the identification system.
    pub radical: char,
    /// The number of strokes not included in the radical.
    pub other_strokes: u8,
    /// The position of the kanji in the sequence described
    /// by the other descriptor parts.
    pub sequence: u8,
}

/// Kanji code from the SKIP system of indexing.
/// http://www.edrdg.org/wwwjdic/SKIP.html
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Skip {
    /// Pattern 1, the kanji can be divided into left and right parts.
    Horizontal(SkipHorizontal),
    /// Pattern 2, the kanji can be divided into top and bottom parts.
    Vertical(SkipVertical),
    /// Pattern 3, the kanji can be divided by an enclosure element.
    Enclosure(SkipEnclosure),
    /// Pattern 4, the cannot be classified by any of the above patterns.
    Solid(SkipSolid),
}

/// Left and right parts of the kanji.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SkipHorizontal {
    /// Number of strokes in the left part.
    pub left: u8,
    /// Number of strokes in the right part.
    pub right: u8,
}

/// Top and bottom parts of the kanji.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SkipVertical {
    /// Number of strokes in the top part.
    pub top: u8,
    /// Number of strokes in the bottom part.
    pub bottom: u8,
}

/// Interior and exterior parts of the kanji.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SkipEnclosure {
    /// Number of strokes in the exterior part.
    pub exterior: u8,
    /// Number of strokes in the interior part.
    pub interior: u8,
}

/// Classification for kanji that don't fit another pattern.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SkipSolid {
    /// The total number of strokes in the kanji.
    pub total_stroke_count: u8,
    /// The subpattern that defines the kanji.
    pub solid_subpattern: SolidSubpattern,
}

/// An identifying characteristic of the kanji.
#[derive(TryFromPrimitive, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SolidSubpattern {
    /// Contains a top line.
    TopLine = 1,
    /// Contains a bottom line.
    BottomLine,
    /// Contains a through line.
    ThroughLine,
    /// Does not contain any of the above.
    Other,
}

/// A particular reading or pronunciation of a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Reading {
    /// The modern romanization of the Chinese reading.
    PinYin(PinYin),
    /// The romanized form of the Korean reading.
    KoreanRomanized(String),
    /// The Korean reading of the kanji in Hangul.
    KoreanHangul(String),
    /// The Vietnamese reading supplied by Minh Chau Pham.
    Vietnam(String),
    /// The onyomi reading of the kanji in katakana.
    Onyomi(String),
    /// The kunyomi reading of the kanji in hiragana or katakana.
    Kunyomi(Kunyomi),
}

/// A kanji classification based on its radical.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Radical {
    /// Based on the KangXi Zidian system.
    /// Referenced from the Shibano JIS Kanwa Jiten.
    Classical(KangXi),
    /// As used in the classic Modern Japanese-English Character Dictionary.
    Nelson(KangXi),
}

/// Information relating to a kanji that can be
/// used for identification and lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum QueryCode {
    /// The Halpern SKIP code
    Skip(Skip),
    /// Desrcriptor codes from The Kanji Dictionary
    SpahnHadamitzky(ShDesc),
    /// The Four Corner code
    FourCorner(FourCorner),
    /// Father Joseph De Roo's code system
    DeRoo(DeRoo),
    /// A possible misclassification of the kanji
    Misclassification(Misclassification),
}

/// A possible misclassification of the kanji
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Misclassification {
    /// A mistake in the division of the kanji
    Position(Skip),
    /// A mistake in the number of strokes
    StrokeCount(Skip),
    /// Mistakes in both the division and the number of strokes
    StrokeAndPosition(Skip),
    /// Ambiguous stroke counts
    Ambiguous(Skip),
}

// A modern PinYin romanization of the Chinese reading.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct PinYin {
    /// The romanized reading.
    pub romanization: String,
    /// The Mandarin tone of the reading.
    pub tone: Tone,
}

/// One of the four tones of Mandarin.
/// https://en.wikipedia.org/wiki/Standard_Chinese_phonology#Tones
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum Tone {
    /// A steady high sound
    High = 1,
    /// A rising tone
    Rising,
    /// A low or dipping tone
    Low,
    /// A sharp falling tone
    Falling,
    /// A lack of tone
    Neutral,
}

/// An entry in the dictionary Daikanwajiten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Moro {
    /// The volume
    pub volume: Option<u8>,
    /// The page
    pub page: Option<u16>,
    /// The reference index
    pub index: MoroIndex,
}

/// The reference index
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MoroIndex {
    /// The item number
    pub number: u16,
    /// A letter that appears after the index
    pub suffix: MoroSuffix,
}

/// A letter that appears at the end of the index
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MoroSuffix {
    /// No suffix
    None,
    /// P suffix
    P,
    /// X suffix
    X,
    /// PX suffix
    PX,
}

/// Information about a particular meaning of a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Meaning {
    /// Different ways the kanji can be read.
    pub readings: Vec<Reading>,
    /// Translations of the kanji into different languages.
    pub translations: Vec<Translation>,
    /// Japanese readings associated with names.
    pub nanori: Vec<String>,
}

/// A kuten representation of a JIS X 0213 character.
/// http://unicode-iphone.blogspot.com/2010/05/kuten-code-to-unicode.html
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Kuten {
    /// The plane on which a kuten representation is found.
    pub plane: u8,
    /// The Ku part of the matrix position.
    pub ku: u8,
    /// The Ten part of the matrix position.
    pub ten: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Kunyomi {
    /// The okurigana
    pub okurigana: Vec<String>,
    /// Whether the reading is as a prefix or suffix.
    pub kind: KunyomiKind,
}

/// The kind of kunyomi reading.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum KunyomiKind {
    /// A normal reading
    Normal,
    /// A prefix
    Prefix,
    /// A suffix
    Suffix,
}

/// The grade level in which the kanji is learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Grade {
    /// A Kyouiku kanji learned in grades 1-6.
    Kyouiku(u8),
    /// A remaining Jouyou kanji to be learned in junior hi-school.
    Jouyou,
    /// A Jinmeiyou kanji for use in names that is approved
    /// for use in family name registers and other official documents.
    Jinmeiyou,
    /// A Jinmeiyou kanji that is a variant of a Jouyou kanji.
    JinmeiyouJouyouVariant,
}

/// A kanji classification using the Four Corner system.
/// http://www.edrdg.org/wwwjdic/FOURCORNER.html
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FourCorner {
    /// The stroke at the top left corner.
    pub top_left: Stroke,
    /// The stroke at the top right corner.
    pub top_right: Stroke,
    /// The stroke at the bottom left corner.
    pub bottom_left: Stroke,
    /// The stroke at the bottom right corner.
    pub bottom_right: Stroke,
    /// Where necessary to differentiate between other
    /// characters with the same strokes, this extra stroke
    /// is found above the bottom right stroke.
    ///
    /// In the database, we only ever see this with the fifth
    /// corner. Still, not including it is technically
    /// allowed, so I include it here for generality.
    pub fifth_corner: Option<Stroke>,
}

/// A stroke shape in the Four Corner system.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum Stroke {
    /// 亠
    Lid,
    /// 一
    LineHorizontal,
    /// ｜
    LineVertical,
    /// 丶
    Dot,
    /// 十
    Cross,
    /// キ
    Skewer,
    /// 口
    Box,
    /// 厂
    Angle,
    /// 八
    Hachi,
    /// 小
    Chiisai,
}

/// An index number into a particular kanji dictionary or reference book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Reference {
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
    OneillNames(Oneill),
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
    /// A Guide to Reading and Writing Japanese by Henshall
    Henshall3(u16),
    /// Tuttle Kanji Cards by Alexander Kask
    TuttleCards(u16),
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

/// Identification of a kanji in the De Roo system.
/// http://www.edrdg.org/wwwjdic/deroo.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DeRoo {
    /// The graphic element that appears at the top of the kanji.
    pub top: ExtremeTop,
    /// The graphic element that appears at the bottom of the kanji.
    pub bottom: ExtremeBottom,
}

#[derive(TryFromPrimitive, Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ExtremeTop {
    // Dot
    Dot = 3,
    RoofDot,
    DottedCliff,
    Altar,
    KanaU,
    Lid,
    Horns,

    // Vertical line
    SmallOnBox,
    Small,
    VerticalLine,
    HandOnTheLeft,
    Cross,
    CrossOnBox,
    KanaKa,
    Woman,
    Tree,
    LetterH,

    // Diagonal line
    KanaNo,
    ManOnTheLeft,
    Thousand,
    ManOnTheTop,
    Cow,
    KanaKu,
    HillTop,
    LeftArrow,
    RoofDiagonalLine,
    X,

    // Horizontal line
    HorizontalLine,
    Fourth,
    Bald,
    Cliff,
    TopLeftCorner,
    TopRightCorner,
    UpsideDownCan,
    Mouth,
    Sun,
    EyeTop,
}

#[derive(TryFromPrimitive, Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ExtremeBottom {
    // Dot
    FourDots = 40,
    Small,
    Water,

    // Left hook
    KanaRi,
    Seal,
    SwordBottom,
    Moon,
    DotlessInch,
    Inch,
    MouthLeftHook,
    BirdBottom,
    Animal,
    BowBottom,
    LeftHook,

    // Vertical line
    VerticalLine,
    Cross,

    // Right hook
    RightHook,
    Legs,
    Heart,
    TasseledSpearBottom,

    // Diagonal line
    KanaNo,

    // Back diagonal line
    SmallPodium,
    BackKanaNo,
    Big,
    Tree,
    SmallSpoon,
    Govern,
    Again,
    WindyAgain,
    Woman,

    // Head bottom
    HeadBottom,

    // Watakushi bottom
    WatakushiBottom,

    // Horizontal line
    HorizontalLine,
    StandingBottom,
    DishBottom,
    BottomCorner,
    Mountain,
    Mouth,
    Sun,
    Eye,
}

/// The code of a kanji in a given character set standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Codepoint {
    /// Encoding in JIS X 0208-1997
    Jis208(Kuten),
    /// Encoding in JIS X 0212-1990
    Jis212(Kuten),
    /// Encoding in JIS X 0213-2000
    Jis213(Kuten),
    /// Unicode character
    Unicode(u32),
}

/// Information about a kanji.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Character {
    /// The character itself.
    pub literal: String,
    /// Alternate encodings for the character.
    pub codepoints: Vec<Codepoint>,
    /// Alternate classifications for the character by radical.
    pub radicals: Vec<Radical>,
    /// The kanji grade level.
    pub grade: Option<Grade>,
    /// The stroke count of the character.
    pub stroke_counts: StrokeCount,
    /// Cross-references to other characters or alternative indexings.
    pub variants: Vec<Variant>,
    /// A ranking of how often the character appears in newspapers.
    pub frequency: Option<u16>,
    /// The kanji's name as a radical if it is one.
    pub radical_names: Vec<String>,
    /// Old JLPT level of the kanji. Based on pre-2010 test levels
    /// that go up to four, not five.
    pub jlpt: Option<u8>,
    /// Indexes into dictionaries and other instructional books
    pub references: Vec<Reference>,
    /// Codes used to identify the kanji
    pub query_codes: Vec<QueryCode>,
    /// Different meanings of the kanji.
    pub meanings: Vec<Meaning>,
}

/// A location in Japanese for Busy People.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BusyPeople {
    /// The volume
    volume: u8,
    /// The chapter
    chapter: Chapter,
}

/// Either the chapter number or chapter A in Japanese for Busy People.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Chapter {
    /// A chapter number.
    Chapter(u8),
    /// Some of the chapter are called "A",
    /// but it isn't specified anywhere what that means.
    A,
}
