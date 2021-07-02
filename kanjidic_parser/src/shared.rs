pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub type NomErr<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NomErrorReason {
    Incomplete,
    Error(nom::error::ErrorKind),
}

impl<'a> From<NomErr<'a>> for NomErrorReason {
    fn from(err: NomErr) -> Self {
        use nom::Err::*;

        match err {
            Incomplete(_) => NomErrorReason::Incomplete,
            Error(e) | Failure(e) => NomErrorReason::Error(e.code),
        }
    }
}
