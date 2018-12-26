use crate::util::parse_error::ParseError;


#[derive(Debug, PartialEq, Eq)]
pub enum GpsQualifier {
    Gps,
    DGps,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DRecord<'a> {
    pub qualifier: GpsQualifier,
    pub station_id: &'a str,
}

impl<'a> DRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert_eq!(line.len(), 6);

        let bytes = line.as_bytes();
        assert_eq!(bytes[0], b'D');

        let qualifier = match bytes[1] {
            b'1' => GpsQualifier::Gps,
            b'2' => GpsQualifier::DGps,
            _ => return Err(ParseError::SyntaxError),
        };

        let station_id = &line[2..6];

        Ok(DRecord { qualifier, station_id })
    }
}

#[cfg(test)]
mod tests {
    use super::{GpsQualifier,DRecord};

    #[test]
    fn drecord_parse() {
        let example_line = "D1ABCD";
        let parsed = DRecord::parse(example_line).unwrap();
        let expected = DRecord { qualifier: GpsQualifier::Gps, station_id: "ABCD" };

        assert_eq!(parsed, expected);
    }
}
