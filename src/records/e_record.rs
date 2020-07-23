use std::fmt;

use crate::util::{DisplayOption, ParseError, Time};

/// Describes an event logged during the flight, associated with the B Record immediately
/// following.
///
/// An official Event needs a B Record with the same timestamp.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ERecord<'a> {
    pub time: Time,
    pub mnemonic: &'a str,
    pub text: Option<&'a str>,
}

impl<'a> ERecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        if line.len() < 10 {
            return Err(ParseError::SyntaxError);
        }
        if !line.bytes().take(10).all(|b| b.is_ascii()) {
            return Err(ParseError::NonASCIICharacters);
        }

        assert_eq!(line.as_bytes()[0], b'E');

        let time = line[1..7].parse()?;
        let mnemonic = &line[7..10];

        let text = if line.len() > 10 {
            Some(&line[10..])
        } else {
            None
        };

        Ok(ERecord {
            time,
            mnemonic,
            text,
        })
    }
}

impl<'a> fmt::Display for ERecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "E{time}{mnemonic}{text}",
            time = self.time,
            mnemonic = self.mnemonic,
            text = DisplayOption(self.text)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erecord_parse() {
        let example_line = "E120515FOOText";
        let parsed = ERecord::parse(example_line).unwrap();
        let expected = ERecord {
            time: Time::from_hms(12, 5, 15),
            mnemonic: "FOO",
            text: Some("Text"),
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn parse_with_invalid_char_boundary() {
        assert!(ERecord::parse("Eⶠ𑛀  ").is_err());
    }

    #[test]
    fn erecord_format() {
        let expected_string = "E120515FOOText";
        let record = ERecord {
            time: Time::from_hms(12, 5, 15),
            mnemonic: "FOO",
            text: Some("Text"),
        };

        assert_eq!(format!("{}", record), expected_string);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in "E\\PC*") {
            ERecord::parse(&s);
        }
    }
}
