// A modern PinYin romanization of the Chinese reading.
pub struct PinYin<'a> {
    /// The romanized reading.
    pub romanization: &'a str,

    /// The Mandarin tone of the reading.
    pub tone: Tone,
}

/// One of the four tones of Mandarin.
/// https://en.wikipedia.org/wiki/Standard_Chinese_phonology#Tones
pub enum Tone {
    /// A steady high sound
    High,

    /// A rising tone
    Rising,

    /// A low or dipping tone
    Low,

    /// A sharp falling tone
    Falling,

    /// A lack of tone
    Neutral,
}
