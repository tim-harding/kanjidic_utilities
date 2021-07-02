pub type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NomErrorReason {
    Incomplete,
    Error(nom::error::ErrorKind),
}

impl From<nom::Err<nom::error::Error<&str>>> for NomErrorReason {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => NomErrorReason::Incomplete,
            nom::Err::Error(e) | nom::Err::Failure(e) => NomErrorReason::Error(e.code),
        }
    }
}
