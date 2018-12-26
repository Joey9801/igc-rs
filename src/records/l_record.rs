use crate::util::parse_error::ParseError;

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lrecord_parse() {
        let sample_string = "LFoo the bar";
        let parsed = LRecord::parse(sample_string).unwrap();
        let expected = LRecord { log_string: "Foo the bar" };

        assert_eq!(parsed, expected);
    }
}
