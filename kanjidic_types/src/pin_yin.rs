use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::*;

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
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    TryFromPrimitive,
    Serialize_repr,
    Deserialize_repr,
)]
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
