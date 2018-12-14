use std::io;
use std::num;


#[derive(Debug)]
pub enum ParseError {
    IOError(io::Error),
    SyntaxError,
    NumberOutOfRange,
}

impl From<num::ParseIntError> for ParseError {
    fn from(_: num::ParseIntError) -> Self {
        ParseError::SyntaxError
    }
}

