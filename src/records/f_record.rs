use std::fmt;

use crate::util::{ParseError, Time};

/// A record indicating a change in the satellite constellation being used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FRecord<'a> {
    pub time: Time,
    pub satellites: SatelliteArray<'a>,
}

impl<'a> FRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        if line.len() < 7 {
            return Err(ParseError::SyntaxError);
        }
        if !line.is_ascii() {
            return Err(ParseError::NonASCIICharacters);
        }

        let time = line[1..7].parse()?;

        let array_str = &line[7..];
        if array_str.len() % 2 != 0 {
            return Err(ParseError::SyntaxError);
        }
        let satellites = SatelliteArray::new(array_str);

        Ok(Self { time, satellites })
    }
}

impl<'a> fmt::Display for FRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "F{}{}", self.time, self.satellites.raw_str)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SatelliteArray<'a> {
    raw_str: &'a str,
}

impl<'a> SatelliteArray<'a> {
    pub fn new(raw_str: &'a str) -> Self {
        assert!(raw_str.len() % 2 == 0);
        Self { raw_str }
    }

    pub fn iter(&self) -> SatelliteArrayIter<'a> {
        SatelliteArrayIter {
            index: 0,
            raw_str: self.raw_str,
        }
    }
}

pub struct SatelliteArrayIter<'a> {
    index: usize,
    raw_str: &'a str,
}

impl<'a> Iterator for SatelliteArrayIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.raw_str.len() {
            return None;
        }

        let ret = Some(&self.raw_str[self.index..(self.index + 2)]);
        self.index += 2;
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frecord_parse() {
        let sample_string = "F095212AABBCCDDEE";
        let parsed_record = FRecord::parse(sample_string).unwrap();
        let expected_record = FRecord {
            time: Time::from_hms(9, 52, 12),
            satellites: SatelliteArray::new("AABBCCDDEE"),
        };

        assert_eq!(parsed_record, expected_record);
    }

    #[test]
    fn frecord_parse_empty() {
        let sample_string = "F135456";
        let parsed_record = FRecord::parse(sample_string).unwrap();
        let expected_record = FRecord {
            time: Time::from_hms(13, 54, 56),
            satellites: SatelliteArray::new(""),
        };

        assert_eq!(parsed_record, expected_record);
    }

    #[test]
    fn parse_with_invalid_char_boundary() {
        assert!(FRecord::parse("Fኲበ᧞").is_err());
    }

    #[test]
    fn satellite_iter() {
        let satellite_array = SatelliteArray::new("AABBCCDDEE");
        assert_eq!(
            vec!["AA", "BB", "CC", "DD", "EE"],
            satellite_array.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn frecord_format() {
        let expected_string = "F095212AABBCCDDEE";
        let record = FRecord {
            time: Time::from_hms(9, 52, 12),
            satellites: SatelliteArray::new("AABBCCDDEE"),
        };

        assert_eq!(format!("{}", record), expected_string);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in "F\\PC*") {
            FRecord::parse(&s);
        }
    }
}
