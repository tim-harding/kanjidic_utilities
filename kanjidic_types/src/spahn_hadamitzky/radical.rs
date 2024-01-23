use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

/// An identifying radical in the Spahn and Hadamitzky classification system.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Radical {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("(ShRadical) Unknown radical")]
    UnknownChar,
}

impl TryFrom<char> for Radical {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Radical::A),
            'b' => Ok(Radical::B),
            'c' => Ok(Radical::C),
            'd' => Ok(Radical::D),
            'e' => Ok(Radical::E),
            'f' => Ok(Radical::F),
            'g' => Ok(Radical::G),
            'h' => Ok(Radical::H),
            'i' => Ok(Radical::I),
            'j' => Ok(Radical::J),
            'k' => Ok(Radical::K),
            'l' => Ok(Radical::L),
            'm' => Ok(Radical::M),
            'n' => Ok(Radical::N),
            'o' => Ok(Radical::O),
            'p' => Ok(Radical::P),
            'q' => Ok(Radical::Q),
            'r' => Ok(Radical::R),
            's' => Ok(Radical::S),
            't' => Ok(Radical::T),
            'u' => Ok(Radical::U),
            'v' => Ok(Radical::V),
            'w' => Ok(Radical::W),
            'x' => Ok(Radical::X),
            'y' => Ok(Radical::Y),
            'z' => Ok(Radical::Z),
            _ => Err(ParseError::UnknownChar),
        }
    }
}
