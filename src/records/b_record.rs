use crate::util::datetime::Time;
use crate::util::coord::RawPosition;
use crate::util::parse_error::ParseError;

/// Possible values for the "fix valid" field of a B Record
#[derive(Debug, PartialEq, Eq)]
pub enum FixValid {
    Valid,
    NavWarning,
}

/// Closely represents a parsed IGC B-Record with minimal post-processing
///
/// Only the fields for { timestamp, lat, lon, fix_valid, pressure_altitude } are stored.
/// Any/all other fields are optional and defined in an I Record, and are not hanled yet.
///
/// The type for the pressure altitude doesn't techincally cover the complete range of
/// representable altitudes in a conformant IGC file, but to exceed it you would have to be higher
/// than 213,000 feet AMSL..
#[derive(Debug, PartialEq, Eq)]
pub struct BRecord {
    pub timestamp: Time,
    pub pos: RawPosition,
    pub fix_valid: FixValid,
    pub pressure_alt: u16,
}

impl BRecord {
    /// Parse an IGC B record string.
    ///
    /// ```
    /// # extern crate igc_rs;
    /// # use igc_rs::{ BRecord, Time };
    /// let record = BRecord::parse("B0941145152265N00032642WA00115").unwrap();
    /// assert_eq!(record.timestamp, Time::from_hms(9, 41, 14));
    /// ```
    pub fn parse(line: &str) -> Result<Self, ParseError> {
        let timestamp = Time::parse(&line[1..7])?;
        let pos = RawPosition::parse_lat_lon(&line[7..24])?;

        let fix_valid = match &line[24..25] {
            "A" => FixValid::Valid,
            "V" => FixValid::NavWarning,
            _ => return Err(ParseError::SyntaxError),
        };

        let pressure_alt = line[25..30].parse::<u16>()?;

        Ok(BRecord { timestamp, pos, fix_valid, pressure_alt })
    }
}

#[cfg(test)]
mod test {
    use crate::util::datetime::Time;
    use crate::util::coord::{Compass,RawCoord,RawPosition};

    use super::{BRecord,FixValid};

    #[test]
    fn simple_brecord_parse() {
        // Only mandatory fields, no optional fields defined in I records.
        let sample_string = "B0941145152265N00032642WA00115";
        let parsed_record = BRecord::parse(sample_string).unwrap();
        let expected = BRecord {
            timestamp: Time { hours: 9, minutes: 41, seconds: 14 },
            pos: RawPosition {
                lat: RawCoord { degrees: 51, minute_thousandths: 52265, sign: Compass::North },
                lon: RawCoord { degrees: 0, minute_thousandths: 32642, sign: Compass::West },
            },
            fix_valid: FixValid::Valid,
            pressure_alt: 115,
        };

        // Assert the fields individually first, to give better error messages if they don't match
        assert_eq!(parsed_record.timestamp, expected.timestamp);
        assert_eq!(parsed_record.pos.lat, expected.pos.lat);
        assert_eq!(parsed_record.pos.lon, expected.pos.lon);
        assert_eq!(parsed_record.fix_valid, expected.fix_valid);
        assert_eq!(parsed_record.pressure_alt, expected.pressure_alt);
        assert_eq!(parsed_record, expected);
    }

}
