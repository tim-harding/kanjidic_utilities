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
