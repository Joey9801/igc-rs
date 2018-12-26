use crate::util::parse_error::ParseError;

/// Represents a single G Record (security)
#[derive(Debug, PartialEq, Eq)]
pub struct GRecord<'a> {
    pub data: &'a str,
}

impl <'a> GRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert_eq!(line.as_bytes()[0], b'G');
        if line.len() > 76 {
            return Err(ParseError::SyntaxError);
        }

        Ok(Self { data: &line[1..] })
    }
}
