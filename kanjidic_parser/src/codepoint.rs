use crate::kuten::{Kuten, PlanarKuten};

pub enum Codepoint {
    Jis208(Kuten),
    Jis212(Kuten),
    Jis213(PlanarKuten),
    Unicode(char),
}
