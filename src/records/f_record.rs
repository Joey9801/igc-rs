use crate::util::{
    datetime::Time,
    parse_error::ParseError
};


/// Represents a single parsed F Record (Satellite constellation)
#[derive(Debug, PartialEq, Eq)]
pub struct FRecord<'a> {
    pub time: Time,
    pub satellites: SatelliteArray<'a>,
}

impl<'a> FRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert!(line.len() >= 7);

        let time = Time::parse(&line[1..7])?;

        let array_str = &line[7..];
        if array_str.len() < 2 || array_str.len() % 2 != 0 {
            return Err(ParseError::SyntaxError);
        }
        let satellites = SatelliteArray::new(array_str);

        Ok(Self { time, satellites })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SatelliteArray<'a> {
    raw_str: &'a str,
}

impl<'a> SatelliteArray<'a> {
    pub fn new(raw_str: &'a str) -> Self {
        assert!(raw_str.len() % 2 == 0);
        Self { raw_str }
    }

    pub fn iter(&self) -> SatelliteArrayIter<'a> {
        SatelliteArrayIter { index: 0, raw_str: self.raw_str }
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

        let ret = Some(&self.raw_str[self.index..(self.index+2)]);
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

        assert_eq!(parsed_record.time, Time::from_hms(9, 52, 12));

        let satellites: Vec<&str> = parsed_record.satellites.iter().collect();
        assert_eq!(satellites, vec!["AA", "BB", "CC", "DD", "EE"]);
    }
}
