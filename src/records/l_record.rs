use std::fmt;

use crate::util::ParseError;

/// A simple plaintext log, used by some manufacturers for propietary extensions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LRecord<'a> {
    pub log_string: &'a str,
}

impl<'a> LRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert_eq!(line.as_bytes()[0], b'L');

        let log_string = &line[1..];

        Ok(Self { log_string })
    }
}

impl<'a> fmt::Display for LRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L{}", self.log_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lrecord_parse() {
        let sample_string = "LFoo the bar";
        let parsed = LRecord::parse(sample_string).unwrap();
        let expected = LRecord {
            log_string: "Foo the bar",
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn lrecord_format() {
        let expected_string = "LFoo the bar";
        let record = LRecord {
            log_string: "Foo the bar",
        };

        assert_eq!(format!("{}", record), expected_string);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in "L\\PC*") {
            LRecord::parse(&s);
        }
    }
}
