use crate::util::parse_error::ParseError; 

#[derive(Debug, PartialEq, Eq)]
pub struct Extension<'a> {
    pub start_byte: u8,
    pub end_byte: u8,
    pub mnemonic: &'a str,
}

impl<'a> Extension<'a> {
    pub fn parse(string: &'a str) -> Result<Self, ParseError> {
        assert_eq!(string.len(), 7);

        let start_byte = string[0..2].parse::<u8>()?;
        let end_byte = string[2..4].parse::<u8>()?;
        let mnemonic = &string[4..7];

        Ok(Self { start_byte, end_byte, mnemonic })
    }
}
