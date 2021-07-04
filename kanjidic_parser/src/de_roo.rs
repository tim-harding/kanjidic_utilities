/// Identification of a kanji in the De Roo system.
/// http://www.edrdg.org/wwwjdic/deroo.html
pub struct DeRoo {
    /// The graphic element that appears at the top of the kanji.
    pub extreme_top: ExtremeTop,

    /// The graphic element that appears at the bottom of the kanji.
    pub extreme_bottom: ExtremeBottom,
}

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
    Mountain,
    Mouth,
    Sun,
    Eye,
}
