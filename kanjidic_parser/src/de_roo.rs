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
