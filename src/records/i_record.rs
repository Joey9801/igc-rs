use crate::util::parse_error::ParseError; 
use records::extension::Extension;
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub struct IRecord<'a> {
    pub num_extensions: u8,
    pub extensions: Vec<Extension<'a>>,
}

impl<'a> IRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert_eq!(line.as_bytes()[0], b'I');
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

        Ok(IRecord { num_extensions, extensions } )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn irecord_parse() {
        let sample_string = "I033638FXA3941ENL4246TAS";
        let parsed_record = IRecord::parse(sample_string).unwrap();
        let expected = IRecord {
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
