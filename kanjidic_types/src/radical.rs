use serde::{Deserialize, Serialize};

use crate::KangXi;

/// A kanji classification based on its radical.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum Radical {
    /// Based on the KangXi Zidian system.
    /// Referenced from the Shibano JIS Kanwa Jiten.
    Classical(KangXi),
    /// As used in the classic Modern Japanese-English Character Dictionary.
    Nelson(KangXi),
}
