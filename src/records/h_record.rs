use crate::util::parse_error::ParseError;

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
}

impl<'a> HRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        let bytes = line.as_bytes();
        assert_eq!(bytes[0], b'H');

        let data_source = DataSource::from_byte(bytes[1]);
        let mnemonic = &line[2..5];

        let friendly_name;
        let data;
        if let Some(colon_idx) = line.find(":") {
            friendly_name = Some(&line[5..colon_idx]);
            data = &line[colon_idx+1..];
        } else {
            friendly_name = None;
            data = &line[5..];
        }

        Ok(Self { data_source, mnemonic, friendly_name, data })
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
            data_source : DataSource::FVU,
            mnemonic : "GID",
            friendly_name : Some("GLIDERID"),
            data: "D-KOOL"
        };

        assert_eq!(parsed_record, expected);
    }
}
