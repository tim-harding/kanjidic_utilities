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
