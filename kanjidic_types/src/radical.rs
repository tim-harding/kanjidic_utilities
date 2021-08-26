use serde::{Deserialize, Serialize};

use crate::KangXi;

/// A kanji classification based on its radical.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Radical {
    /// The kind of radical classification
    pub kind: RadicalKind,
    /// The kang xi code for the radical
    pub radical: KangXi,
}

/// The kind of kanji classification
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RadicalKind {
    /// Based on the KangXi Zidian system.
    /// Referenced from the Shibano JIS Kanwa Jiten.
    Classical,
    /// As used in the classic Modern Japanese-English Character Dictionary.
    Nelson,
}
