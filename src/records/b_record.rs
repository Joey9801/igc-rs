use crate::util::{Time,RawPosition,ParseError};
use crate::records::extension::Extendable;


/// Possible values for the "fix valid" field of a B Record
#[derive(Debug, PartialEq, Eq)]
pub enum FixValid {
    Valid,
    NavWarning,
}

/// A Fix record
///
/// Only the fields for { timestamp, lat, lon, fix_valid, pressure_altitude, gps_altitude} are stored.
/// Any/all other fields are optional and defined in an I Record, and are not hanled yet.
///
/// The type for the altitudes doesn't techincally cover the complete range of representable
/// altitudes in a conformant IGC file, but to exceed it you would have to beat the Perlan
/// Project's objective altitude (90,000ft, unachieved at the time of writing) by >15,000ft.
#[derive(Debug, PartialEq, Eq)]
pub struct BRecord<'a> {
    pub timestamp: Time,
    pub pos: RawPosition,
    pub fix_valid: FixValid,
    pub pressure_alt: i16,
    pub gps_alt: i16,
    extension_string: &'a str,
}

impl<'a> BRecord<'a> {
    /// Parse an IGC B record string.
    ///
    /// ```
    /// # use igc::{ records::BRecord, util::Time };
    /// let record = BRecord::parse("B0941145152265N00032642WA0011500115").unwrap();
    /// assert_eq!(record.timestamp, Time::from_hms(9, 41, 14));
    /// ```
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        if line.len() < Self::BASE_LENGTH {
            return Err(ParseError::SyntaxError);
        }

        let timestamp = line[1..7].parse()?;
        let pos = line[7..24].parse()?;

        let fix_valid = match &line[24..25] {
            "A" => FixValid::Valid,
            "V" => FixValid::NavWarning,
            _ => return Err(ParseError::SyntaxError),
        };

        let pressure_alt = line[25..30].parse::<i16>()?;
        let gps_alt = line[30..35].parse::<i16>()?;

        let extension_string = &line[35..];

        Ok(Self { timestamp, pos, fix_valid, pressure_alt, gps_alt, extension_string })
    }
}

impl<'a> Extendable for BRecord<'a> {
    const BASE_LENGTH: usize = 35;

    fn extension_string(&self) -> &str {
        self.extension_string
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::{Time,Compass,RawCoord,RawPosition};
    use crate::records::extension::Extension;

    #[test]
    fn simple_brecord_parse() {
        // Only mandatory fields, no optional fields defined in I records.
        let sample_string = "B0941145152265N00032642WA00115-0116FooExtensionString";
        let parsed_record = BRecord::parse(sample_string).unwrap();
        let expected = BRecord {
            timestamp: Time::from_hms(9, 41, 14),
            pos: RawPosition {
                lat: RawCoord { degrees: 51, minute_thousandths: 52265, sign: Compass::North },
                lon: RawCoord { degrees: 0, minute_thousandths: 32642, sign: Compass::West },
            },
            fix_valid: FixValid::Valid,
            pressure_alt: 115,
            gps_alt: -116,
            extension_string: "FooExtensionString",
        };

        // Assert the fields individually first, to give better error messages if they don't match
        assert_eq!(parsed_record.timestamp, expected.timestamp);
        assert_eq!(parsed_record.pos.lat, expected.pos.lat);
        assert_eq!(parsed_record.pos.lon, expected.pos.lon);
        assert_eq!(parsed_record.fix_valid, expected.fix_valid);
        assert_eq!(parsed_record.pressure_alt, expected.pressure_alt);
        assert_eq!(parsed_record, expected);
    }

    #[test]
    fn brecord_get_extension() {
        let record = BRecord {
            timestamp: Time::from_hms(9, 41, 14),
            pos: RawPosition {
                lat: RawCoord { degrees: 51, minute_thousandths: 52265, sign: Compass::North },
                lon: RawCoord { degrees: 0, minute_thousandths: 32642, sign: Compass::West },
            },
            fix_valid: FixValid::Valid,
            pressure_alt: 115,
            gps_alt: 116,
            extension_string: "0123456789",
        };

        let extension = Extension { start_byte: 36, end_byte: 40, mnemonic: "FOO" };

        let extracted = record.get_extension(&extension).unwrap();
        let expected = "01234";
        assert_eq!(extracted, expected);
    }
}
