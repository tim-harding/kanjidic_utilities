/// A kanji classification using the Four Corner system.
/// http://www.edrdg.org/wwwjdic/FOURCORNER.html
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
    pub fifth_corner: Option<Stroke>,
}

/// A stroke shape in the Four Corner system.
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
