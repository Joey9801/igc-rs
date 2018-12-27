use std::io;
use std::num;


/// Enumeration of different errors that can occur during parsing
#[derive(Debug)]
pub enum ParseError {
    IOError(io::Error),
    Utf8Error(std::str::Utf8Error),
    SyntaxError,
    NumberOutOfRange,
    BadExtension,
    MissingExtension,
}

impl From<num::ParseIntError> for ParseError {
    fn from(_: num::ParseIntError) -> Self {
        ParseError::SyntaxError
    }
}

impl From<std::str::Utf8Error> for ParseError {
    fn from (e: std::str::Utf8Error) -> Self {
        ParseError::Utf8Error(e)
    }
}
