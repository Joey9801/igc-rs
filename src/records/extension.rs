use std::{fmt, str};

use crate::util::ParseError;

/// Defines a generic record extension, as appears in I and J records.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Extension<'a> {
    pub range: ExtensionRange,
    pub mnemonic: &'a str,
}

impl<'a> Extension<'a> {
    pub const STRING_LENGTH: usize = 7;

    /// Parse an single extension definition string.
    ///
    /// Expected format:
    ///     SSEEMMM
    /// SS  - start byte - 0-9
    /// EE  - end byte   - 0-9
    /// MMM - mnemonic   - 0-9 a-z A-Z
    pub fn parse(string: &'a str) -> Result<Self, ParseError> {
        if string.len() != 7 {
            return Err(ParseError::SyntaxError);
        }

        let range = string[0..4].parse()?;
        let mnemonic = &string[4..7];

        Ok(Self { range, mnemonic })
    }
}

impl<'a> fmt::Display for Extension<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.range, self.mnemonic)
    }
}

/// Defines the range part of a  generic record extension, as appears in I and J records.
///
/// The start and end bytes are defined as being 1-indexed including the initial record type
/// discrimination character.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExtensionRange {
    pub start_byte: u8,
    pub end_byte: u8,
}

impl ExtensionRange {
    /// Parse an single extension definition range.
    ///
    /// Expected format:
    ///     SSEE
    /// SS  - start byte - 0-9
    /// EE  - end byte   - 0-9
    pub fn parse(string: &str) -> Result<Self, ParseError> {
        if string.len() != 4 {
            return Err(ParseError::SyntaxError);
        }

        let start_byte = string[0..2].parse::<u8>()?;
        let end_byte = string[2..4].parse::<u8>()?;

        if end_byte < start_byte {
            return Err(ParseError::BadExtension);
        }

        Ok(Self {
            start_byte,
            end_byte,
        })
    }
}

impl str::FromStr for ExtensionRange {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        Self::parse(s)
    }
}

impl<'a> fmt::Display for ExtensionRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}{:02}", self.start_byte, self.end_byte,)
    }
}

/// Implemented by records which support having extensions
pub trait Extendable {
    const BASE_LENGTH: usize;

    fn extension_string(&self) -> &str;

    /// Get a given extension from the record implementing this trait.
    fn get_extension(&self, range: ExtensionRange) -> Result<&str, ParseError> {
        if (range.start_byte as usize) < Self::BASE_LENGTH {
            return Err(ParseError::BadExtension);
        }

        let ext_str = self.extension_string();

        // The start/end bytes are specified as being 1-indexed
        let start = range.start_byte as usize - Self::BASE_LENGTH - 1;
        let end = range.end_byte as usize - Self::BASE_LENGTH;

        if start >= ext_str.len() {
            Err(ParseError::MissingExtension)
        } else {
            Ok(&ext_str[start..end])
        }
    }
}

/// A record defining a set of extensions (either an I or a J record)
#[derive(Debug, PartialEq, Eq)]
pub struct ExtensionDefRecord<'a> {
    pub num_extensions: u8,
    pub extensions: Vec<Extension<'a>>,
}

impl<'a> ExtensionDefRecord<'a> {
    /// Parse either kind of extension definition records (either I or J)
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

        let extensions = line[3..]
            .as_bytes()
            .chunks(Extension::STRING_LENGTH)
            .map(unsafe { |buf| str::from_utf8_unchecked(buf) })
            .map(Extension::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            num_extensions,
            extensions,
        })
    }

    fn fmt(&self, f: &mut fmt::Formatter, letter: char) -> fmt::Result {
        write!(f, "{}{:02}", letter, self.num_extensions)?;
        for ext in self.extensions.iter() {
            write!(f, "{}", ext)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IRecord<'a>(pub ExtensionDefRecord<'a>);

impl<'a> IRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        let first_byte = line.as_bytes()[0];
        assert!(first_byte == b'I');
        Ok(IRecord(ExtensionDefRecord::parse(line)?))
    }
}

impl<'a> fmt::Display for IRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f, 'I')
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct JRecord<'a>(pub ExtensionDefRecord<'a>);

impl<'a> JRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        let first_byte = line.as_bytes()[0];
        assert!(first_byte == b'J');
        Ok(JRecord(ExtensionDefRecord::parse(line)?))
    }
}

impl<'a> fmt::Display for JRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f, 'J')
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
                Extension {
                    mnemonic: "FXA",
                    range: ExtensionRange {
                        start_byte: 36,
                        end_byte: 38,
                    },
                },
                Extension {
                    mnemonic: "ENL",
                    range: ExtensionRange {
                        start_byte: 39,
                        end_byte: 41,
                    },
                },
                Extension {
                    mnemonic: "TAS",
                    range: ExtensionRange {
                        start_byte: 42,
                        end_byte: 46,
                    },
                },
            ],
        };

        assert_eq!(parsed_record, expected);
    }

    #[test]
    fn irecord_format() {
        let expected_string = "I033638FXA3941ENL4246TAS";
        let record = IRecord(ExtensionDefRecord {
            num_extensions: 3,
            extensions: vec![
                Extension {
                    mnemonic: "FXA",
                    range: ExtensionRange {
                        start_byte: 36,
                        end_byte: 38,
                    },
                },
                Extension {
                    mnemonic: "ENL",
                    range: ExtensionRange {
                        start_byte: 39,
                        end_byte: 41,
                    },
                },
                Extension {
                    mnemonic: "TAS",
                    range: ExtensionRange {
                        start_byte: 42,
                        end_byte: 46,
                    },
                },
            ],
        });

        assert_eq!(format!("{}", record), expected_string);
    }

    #[test]
    fn jrecord_format() {
        let expected_string = "J033638FXA3941ENL4246TAS";
        let record = JRecord(ExtensionDefRecord {
            num_extensions: 3,
            extensions: vec![
                Extension {
                    mnemonic: "FXA",
                    range: ExtensionRange {
                        start_byte: 36,
                        end_byte: 38,
                    },
                },
                Extension {
                    mnemonic: "ENL",
                    range: ExtensionRange {
                        start_byte: 39,
                        end_byte: 41,
                    },
                },
                Extension {
                    mnemonic: "TAS",
                    range: ExtensionRange {
                        start_byte: 42,
                        end_byte: 46,
                    },
                },
            ],
        });

        assert_eq!(format!("{}", record), expected_string);
    }
}
