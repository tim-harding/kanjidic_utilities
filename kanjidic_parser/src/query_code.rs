use crate::{
    de_roo::DeRoo, four_corner::FourCorner, skip::Skip, spahn_hadamitzky::SpahnHadamitzkyDescriptor,
};

pub enum QueryCode {
    Skip(Skip),
    SpahnHadamitzky(SpahnHadamitzkyDescriptor),
    FourCorner(FourCorner),
    DeRoo(DeRoo),
    Misclassification(Misclassification),
}

pub enum Misclassification {
    Position(Skip),
    StrokeCount(Skip),
    StrokeAndPosition(Skip),
    Ambiguous(Skip),
}
