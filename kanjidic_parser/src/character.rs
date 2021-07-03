use iso639_1::Iso639_1;

pub struct Character<'a> {
    pub literal: Literal<'a>,
    pub codepoints: Vec<Codepoint>,
    pub radicals: Vec<Radical>,
    pub grade: Grade,
    pub stroke_count: StrokeCount,
    pub variants: Vec<Variant>,
    pub radical_names: Vec<RadicalName<'a>>,
    pub jlpt: Option<Jlpt>,
}

pub struct Jlpt(pub u8);

pub struct RadicalName<'a>(pub &'a str);

pub struct Frequency(pub u16);

pub struct Literal<'a>(pub &'a str);

// http://unicode-iphone.blogspot.com/2010/05/kuten-code-to-unicode.html
// http://nihongo.monash.edu/coding_inf.htm
pub enum Codepoint {
    Jis208(Kuten),
    Jis212(Kuten),
    Jis213(PlanarKuten),
    Unicode(char),
}

pub struct Kuten {
    pub ku: u8,
    pub ten: u8,
}

pub struct PlanarKuten {
    pub plane: u8,
    pub ku: u8,
    pub ten: u8,
}

pub enum Radical {
    Classical(u8),
    Nelson(u8),
}

pub struct StrokeCount {
    pub accepted: u8,
    pub miscounts: Vec<u8>,
}

pub struct Grade(pub Option<u8>);

pub enum Variant {
    Jis208(Kuten),
    Jis212(Kuten),
    Jis213(PlanarKuten),
    Unicode(char),
    DeRoo(u16),
    Halpern(u16),
    SpahnHadamitzky(SpahnHadamitzkyDescriptor),
    Nelson(u16),
    ONeill(u16),
}

pub struct SpahnHadamitzkyDescriptor {
    pub radical_strokes: u8,
    pub radical: char,
    pub other_strokes: u8,
    pub sequence: u8,
}

// dic_number?
pub enum DictionaryReference {
    NelsonClassic(u16),
    NelsonNew(u16),
    Njecd(u16),
    Kkd(u16),
    Kkld(u16),
    Kkld2ed(u16),
    Heisig(u16),
    Heisig6(u16),
    Gakken(u16),
    OneillNames(u16),
    OneillKk(u16),
    Moro(Moro),
    Henshall(u16),
    ShKk(u16),
    ShKk2(u16),
    Sakade(u16),
    Jfcards(u16),
    Crowley(u16),
    KanjiInContext(u16),
    BusyPeople(u16),
    KodanshaCompact(u16),
    Maniette(u16),
}

pub struct Moro {
    pub volume: Option<u8>,
    pub page: Option<u16>,
    pub item: u16,
}

// query_code?
pub enum QueryCode {
    Skip(Skip),
    SpahnHadamitzky(SpahnHadamitzkyDescriptor),
    FourCorner(FourCorner),
    DeRoo(DeRoo),    
    Misclassification(Misclassification),
}

pub enum Misclassification {
    Position(Skip),
    StrokeCount(Skip),
    StrokeAndPosition(Skip),
    Ambiguous(Skip),
}

// http://www.edrdg.org/wwwjdic/deroo.html
pub struct DeRoo {
    extreme_top: ExtremeTop,
    extreme_bottom: ExtremeBottom,
}

pub enum ExtremeTop {
    Dot = 3,
    RoofDot,
    DottedCliff,
    Altar,
    KanaU,
    Lid,
    Horns,
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

pub enum ExtremeBottom {
    FourDots,
    Small,
    Water,
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
    VerticalLine,
    Cross,
    RightHook,
    Legs,
    Heart,
    TasseledSpearBottom,
    KanaNo,
    SmallPodium,
    BackKanaNo,
    Big,
    Tree,
    SmallSpoon,
    Govern,
    Again,
    WindyAgain,
    Woman,
    HeadBottom,
    WatakushiBottom,
    HorizontalLine,
    StandingBottom,
    DishBottom,
    Mountain,
    Mouth,
    Sun,
    Eye,
}

// http://www.edrdg.org/wwwjdic/FOURCORNER.html
pub struct FourCorner {
    top_left: Stroke,
    top_right: Stroke,
    bottom_left: Stroke,
    bottom_right: Stroke,
    fifth_corner: Stroke,
}

pub enum Stroke {
    Lid,
    LineHorizontal,
    LineVertical,
    Dot,
    Cross,
    Skewer,
    Box,
    Angle,
    Hachi,
    Chiisai,
}

// http://www.edrdg.org/wwwjdic/SKIP.html
pub enum Skip {
    Horizontal(SkipHorizontal),
    Vertical(SkipVertical),
    Enclosure(SkipEnclosure),
    Solid(SkipSolid),
}

pub struct SkipHorizontal {
    left: u8,
    right: u8,
}

pub struct SkipVertical {
    top: u8,
    bottom: u8,
}

pub struct SkipEnclosure {
    exterior: u8,
    interior: u8,
}

pub struct SkipSolid {
    total_stroke_count: u8,
    solid_subpattern: SolidSubpattern,
}

pub enum SolidSubpattern {
    TopLine = 1,
    BottomLine = 2,
    ThroughLine = 3,
    Other = 4,
}

// reading_meaning?
pub struct Meaning<'a> {
    pub readings: Vec<Reading<'a>>,
    pub translations: Vec<Translation<'a>>,
    pub nanori: Vec<&'a str>,
}

pub enum Reading<'a> {
    PinYin(PinYin<'a>),
    KoreanRomanized(&'a str),
    KoreanHangul(&'a str),
    Vietnam(&'a str),
    Onyomi(&'a str),
    Kunyomi(Kunyomi<'a>)
}

pub struct Kunyomi<'a> {
    okurigana: Vec<&'a str>,
    kind: KunyomiKind,
}

pub enum KunyomiKind {
    Normal,
    Prefix,
    Suffix,
}

pub struct PinYin<'a> {
    romanization: &'a str,
    tone: Tone,
}

pub enum Tone {
    High,
    Rising,
    Low,
    Falling,
    Neutral,
}

pub struct Translation<'a> {
    text: &'a str,
    language: Iso639_1,
}