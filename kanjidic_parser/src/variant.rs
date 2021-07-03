use crate::{
    kuten::{Kuten, PlanarKuten},
    spahn_hadamitzky::SpahnHadamitzkyDescriptor,
};

pub enum Variant {
    Jis208(Kuten),
    Jis212(Kuten),
    Jis213(PlanarKuten),
    Unicode(char),
    DeRoo(u16),
    Halpern(u16),
    SpahnHadamitzky(SpahnHadamitzkyDescriptor),
    Nelson(u16),
    ONeill(u16),
}
