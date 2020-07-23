use std::{fmt, str};

use crate::records::extension::ExtensionDefRecord;
use crate::util::ParseError;

#[derive(Clone, Debug, PartialEq, Eq)]
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
    use crate::records::extension::Extension;

    #[test]
    fn jrecord_format() {
        let expected_string = "J033638FXA3941ENL4246TAS";
        let record = JRecord(ExtensionDefRecord {
            num_extensions: 3,
            extensions: vec![
                Extension::new("FXA", 36, 38),
                Extension::new("ENL", 39, 41),
                Extension::new("TAS", 42, 46),
            ],
        });

        assert_eq!(format!("{}", record), expected_string);
    }
}
