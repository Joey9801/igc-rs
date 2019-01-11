use std::fmt;

use crate::util::ParseError;

/// Enumeration of the different sources an H record can come from.
#[derive(Debug, PartialEq, Eq)]
pub enum DataSource {
    FVU,
    OfficialObserver,
    Pilot,
    Unrecognized(u8),
}

/// A header information record.
#[derive(Debug, PartialEq, Eq)]
pub struct HRecord<'a> {
    pub data_source: DataSource,
    pub mnemonic: &'a str,
    pub friendly_name: Option<&'a str>,
    pub data: &'a str,
}

impl DataSource {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'F' => DataSource::FVU,
            b'O' => DataSource::OfficialObserver,
            b'P' => DataSource::Pilot,
            _ => DataSource::Unrecognized(byte),
        }
    }

    fn to_byte(&self) -> u8 {
        match self {
            DataSource::FVU => b'F',
            DataSource::OfficialObserver => b'O',
            DataSource::Pilot => b'P',
            DataSource::Unrecognized(byte) => *byte,
        }
    }
}

impl<'a> HRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        let bytes = line.as_bytes();
        assert_eq!(bytes[0], b'H');

        if bytes.len() < 6 {
            return Err(ParseError::SyntaxError);
        }
        let data_source = DataSource::from_byte(bytes[1]);
        let mnemonic = &line[2..5];

        let friendly_name;
        let data;
        if let Some(colon_idx) = line.find(':') {
            friendly_name = Some(&line[5..colon_idx]);
            data = &line[colon_idx + 1..];
        } else {
            friendly_name = None;
            data = &line[5..];
        }

        Ok(Self {
            data_source,
            mnemonic,
            friendly_name,
            data,
        })
    }
}

impl<'a> fmt::Display for HRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // NB: not using DisplayOption, as the colon also disappears when friendly_name is None
        match self.friendly_name {
            Some(friendly_name) => write!(
                f,
                "H{source}{mnemonic}{friendly_name}:{data}",
                source = self.data_source.to_byte() as char,
                mnemonic = self.mnemonic,
                friendly_name = friendly_name,
                data = self.data
            ),
            None => write!(
                f,
                "H{source}{mnemonic}{data}",
                source = self.data_source.to_byte() as char,
                mnemonic = self.mnemonic,
                data = self.data
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hrecord_parse() {
        let sample_string = "HFGIDGLIDERID:D-KOOL";
        let parsed_record = HRecord::parse(sample_string).unwrap();
        let expected = HRecord {
            data_source: DataSource::FVU,
            mnemonic: "GID",
            friendly_name: Some("GLIDERID"),
            data: "D-KOOL",
        };

        assert_eq!(parsed_record, expected);
    }

    #[test]
    fn parse_with_missing_content() {
        assert!(HRecord::parse("H").is_err());
        assert!(HRecord::parse("HXXX").is_err());
    }

    #[test]
    fn hrecord_format() {
        let expected_string = "HFGIDGLIDERID:D-KOOL";
        let record = HRecord {
            data_source: DataSource::FVU,
            mnemonic: "GID",
            friendly_name: Some("GLIDERID"),
            data: "D-KOOL",
        };

        assert_eq!(format!("{}", record), expected_string);
    }
}
