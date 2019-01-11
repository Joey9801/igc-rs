use std::fmt;

use crate::records::extension::Extendable;
use crate::util::{ParseError, Time};

/// An extension data record.
///
/// Contains only a timestamp by default, but can be extended with a J record.
#[derive(Debug, PartialEq, Eq)]
pub struct KRecord<'a> {
    pub time: Time,
    extension_string: &'a str,
}

impl<'a> KRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert_eq!(line.as_bytes()[0], b'K');

        if line.len() <= 7 {
            return Err(ParseError::SyntaxError);
        }

        let time = line[1..7].parse()?;
        let extension_string = &line[7..];

        Ok(Self {
            time,
            extension_string,
        })
    }
}

impl<'a> Extendable for KRecord<'a> {
    const BASE_LENGTH: usize = 7;

    fn extension_string(&self) -> &str {
        self.extension_string
    }
}

impl<'a> fmt::Display for KRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "K{}{}", self.time, self.extension_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::extension::{Extension, ExtensionRange};

    #[test]
    fn krecord_parse() {
        let sample_string = "K095214FooTheBar";
        let parsed = KRecord::parse(sample_string).unwrap();
        let expected = KRecord {
            time: Time::from_hms(9, 52, 14),
            extension_string: "FooTheBar",
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn krecord_format() {
        let expected_string = "K095214FooTheBar";
        let record = KRecord {
            time: Time::from_hms(9, 52, 14),
            extension_string: "FooTheBar",
        };

        assert_eq!(format!("{}", record), expected_string);
    }

    #[test]
    fn krecord_extensions() {
        let record = KRecord {
            time: Time::from_hms(9, 52, 14),
            extension_string: "FooTheBar",
        };
        let ext1 = Extension::new("One", 8, 10);
        let ext2 = Extension::new("Two", 11, 13);
        let ext3 = Extension::new("Th3", 14, 16);

        assert_eq!(record.get_extension(ext1.range).unwrap(), "Foo");
        assert_eq!(record.get_extension(ext2.range).unwrap(), "The");
        assert_eq!(record.get_extension(ext3.range).unwrap(), "Bar");
    }
}
