use crate::util::ParseError;

/// A security record.
///
/// The contents of the record are vendor dependent.
#[derive(Debug, PartialEq, Eq)]
pub struct GRecord<'a> {
    pub data: &'a str,
}

impl <'a> GRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert_eq!(line.as_bytes()[0], b'G');

        Ok(Self { data: &line[1..] })
    }
}
