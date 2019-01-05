use crate::util::{ParseError, Time};

/// Describes an event logged during the flight, associated with the B Record immediately
/// following.
///
/// An official Event needs a B Record with the same timestamp.
#[derive(Debug, PartialEq, Eq)]
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
}
