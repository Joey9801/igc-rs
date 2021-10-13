use std::io;
use std::num;

use thiserror::Error;

/// Enumeration of different errors that can occur during parsing
#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Syntax error found")]
    SyntaxError,
    #[error("Non-ASCII characters found")]
    NonASCIICharacters,
    #[error("Invalid number found")]
    NumberOutOfRange,
    #[error("Invalid extension record found")]
    BadExtension,
    #[error("Extension record missing")]
    MissingExtension,
}

impl From<num::ParseIntError> for ParseError {
    fn from(_: num::ParseIntError) -> Self {
        ParseError::SyntaxError
    }
}
