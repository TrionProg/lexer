use std;
use Line;

///Describes what error has been occured and where
#[derive(Debug)]
pub enum Error{
    UnexpectedEOF(Line,&'static str),
    UnexpectedSymbal(Line, char),
    ParseNumberError(Line, String, std::num::ParseIntError),
    Other(Line,String),
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self{
            Error::UnexpectedEOF(ref line,expected) => write!(f, "Expeced {} , but EOF has been found\n{}", expected, line),
            Error::UnexpectedSymbal(ref line, symbal) => write!(f, "Unexpected symbal {}\n{}", symbal, line),
            Error::ParseNumberError(ref line, ref string, ref e) => write!(f, "Can not parse \"{}\" as number:{}\n{}", string, e, line),
            Error::Other(ref line, ref message) => write!(f, "{}\n{}", message, line),
        }
    }
}

//TODO:impl std::error::Error for Error {
