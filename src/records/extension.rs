use crate::util::parse_error::ParseError; 
use std::str;

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

#[derive(Debug, PartialEq, Eq)]
pub struct ExtensionDefRecord<'a> {
    pub num_extensions: u8,
    pub extensions: Vec<Extension<'a>>,
}

impl<'a> ExtensionDefRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        let first_byte = line.as_bytes()[0];
        assert!(first_byte == b'I' || first_byte == b'J');

        if line.len() < 3 {
            return Err(ParseError::SyntaxError);
        }

        let num_extensions = line[1..3].parse::<u8>()?;

        if line.len() != 3 + (Extension::STRING_LENGTH * num_extensions as usize) {
            return Err(ParseError::SyntaxError);
        }

        let extensions = line[3..].as_bytes()
            .chunks(Extension::STRING_LENGTH)
            .map(unsafe { |buf| str::from_utf8_unchecked(buf) })
            .map(Extension::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { num_extensions, extensions } )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extensiondefrecord_parse() {
        let sample_string = "I033638FXA3941ENL4246TAS";
        let parsed_record = ExtensionDefRecord::parse(sample_string).unwrap();
        let expected = ExtensionDefRecord {
            num_extensions: 3,
            extensions: vec![
                Extension { mnemonic: "FXA", start_byte: 36, end_byte: 38 },
                Extension { mnemonic: "ENL", start_byte: 39, end_byte: 41 },
                Extension { mnemonic: "TAS", start_byte: 42, end_byte: 46 },
            ]
        };

        assert_eq!(parsed_record, expected);
    }
}
