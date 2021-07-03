/// A kanji classification based on its radical.
pub enum Radical {
    /// Based on the KangXi Zidian system.
    /// Referenced from the Shibano JIS Kanwa Jiten.
    Classical(u8),

    /// As used in the classic Modern Japanese-English Character Dictionary.
    Nelson(u8),
}
