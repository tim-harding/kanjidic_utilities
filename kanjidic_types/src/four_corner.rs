use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::*;

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
#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    TryFromPrimitive,
    PartialOrd,
    Ord,
    Serialize_repr,
    Deserialize_repr,
)]
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
