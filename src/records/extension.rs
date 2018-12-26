use crate::util::parse_error::ParseError; 

#[derive(Debug, PartialEq, Eq)]
pub struct Extension<'a> {
    pub start_byte: u8,
    pub end_byte: u8,
    pub mnemonic: &'a str,
}

impl<'a> Extension<'a> {
    pub const STRING_LENGTH: usize = 7;

    pub fn parse(string: &'a str) -> Result<Self, ParseError> {
        assert_eq!(string.len(), 7);

        let start_byte = string[0..2].parse::<u8>()?;
        let end_byte = string[2..4].parse::<u8>()?;

        if end_byte < start_byte {
            return Err(ParseError::BadExtension);
        }

        let mnemonic = &string[4..7];

        Ok(Self { start_byte, end_byte, mnemonic })
    }
}

pub trait Extendable {
    const BASE_LENGTH: usize;

    fn extension_string<'a>(&'a self) -> &'a str;

    fn get_extension<'a, 'b>(&'a self, extension: &'b Extension<'a>) -> Result<&'a str, ParseError> {
        if (extension.start_byte as usize) < Self::BASE_LENGTH {
            return Err(ParseError::BadExtension);
        }

        let ext_str = self.extension_string();

        // The start/end bytes are specified as being 1-indexed
        let start = extension.start_byte as usize - Self::BASE_LENGTH - 1;
        let end = extension.end_byte as usize - Self::BASE_LENGTH;

        if start >= ext_str.len() {
            Err(ParseError::MissingExtension)
        } else {
            Ok(&ext_str[start..end])
        }
    }
}
