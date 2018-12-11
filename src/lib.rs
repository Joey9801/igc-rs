use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::num;


#[derive(Debug)]
pub enum ParseError {
    IOError(io::Error),
    SyntaxError,
    NumberOutOfRange,
}

impl From<num::ParseIntError> for ParseError {
    fn from(_: num::ParseIntError) -> Self {
        ParseError::SyntaxError
    }
}

/// Represents a specific time of day with second precision.
///
/// Does not contain any timezone information as the IGC specification mandates UTC everywhere.
#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
}

impl Time {
    /// Parse a time string of the form "HHMMSS"
    fn parse(time_string: &str) -> Result<Self, ParseError> {
        assert_eq!(time_string.len(), 6);

        let hours = time_string[0..2].parse::<u8>()?;
        let minutes = time_string[2..4].parse::<u8>()?;
        let seconds = time_string[4..6].parse::<u8>()?;

        Ok(Time { hours, minutes, seconds })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Compass {
    North,
    South,
    East,
    West,
}

/// Represents a raw coordinate, as it appears in an IGC file.
#[derive(Debug, PartialEq, Eq)]
pub struct RawCoord {
    degrees: u8,           // in range (0, 90) for lat, (0, 180) for lon
    minutes: u8,           // in range (0, 60)
    minutes_fraction: u16, // Thousandths of a minute, in range(0, 1000)
    sign: Compass,
}

impl RawCoord {
    /// Parse a latitude string of the form "DDMMMMMS"
    fn parse_lat(lat_string: &str) -> Result<Self, ParseError> {
        assert_eq!(lat_string.len(), 8);

        let degrees = lat_string[0..2].parse::<u8>()?;
        let minutes = lat_string[2..4].parse::<u8>()?;
        let minutes_fraction = lat_string[4..7].parse::<u16>()?;
        let sign = match &lat_string[7..8] {
            "N" => Compass::North,
            "S" => Compass::South,
            _ => return Err(ParseError::SyntaxError),
        };

        if degrees > 90 || minutes > 60 || minutes_fraction > 999 {
            Err(ParseError::NumberOutOfRange)
        } else {
            Ok(RawCoord { degrees, minutes, minutes_fraction, sign })
        }
    }

    /// Parse a longitude string of the form "DDDMMMMMW"
    fn parse_lon(lat_string: &str) -> Result<Self, ParseError> {
        assert_eq!(lat_string.len(), 9);

        let degrees = lat_string[0..3].parse::<u8>()?;
        let minutes = lat_string[3..5].parse::<u8>()?;
        let minutes_fraction = lat_string[5..8].parse::<u16>()?;
        let sign = match &lat_string[8..9] {
            "E" => Compass::East,
            "W" => Compass::West,
            _ => return Err(ParseError::SyntaxError),
        };

        if degrees > 180 || minutes > 60 || minutes_fraction > 999 {
            Err(ParseError::NumberOutOfRange)
        } else {
            Ok(RawCoord { degrees, minutes, minutes_fraction, sign })
        }
    }
}

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
    pub lat: RawCoord,
    pub lon: RawCoord,
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
    /// assert_eq!(record.timestamp, Time { hours: 9, minutes: 41, seconds: 14 });
    /// ```
    pub fn parse(line: &str) -> Result<Self, ParseError> {
        let timestamp = Time::parse(&line[1..7])?;
        let lat = RawCoord::parse_lat(&line[7..15])?;
        let lon = RawCoord::parse_lon(&line[15..24])?;

        let fix_valid = match &line[24..25] {
            "A" => FixValid::Valid,
            "V" => FixValid::NavWarning,
            _ => return Err(ParseError::SyntaxError),
        };

        let pressure_alt = line[25..30].parse::<u16>()?;

        Ok(BRecord { timestamp, lat, lon, fix_valid, pressure_alt })
    }
}


/// Closely represents a parsed IGC file, with minimal post-processing
pub struct IGCFile {
    pub filepath: PathBuf,
    pub fixes: Vec<BRecord>,
}

impl IGCFile {
    fn _new(filepath: &Path) -> Self {
        IGCFile {
            filepath: filepath.to_path_buf(),
            fixes: Vec::<BRecord>::new(),
        }
    }

    fn _parse_line(&mut self, line: &str) -> Result<(), ParseError> {
        match line.chars().next().unwrap() {
            'B' => self.fixes.push(BRecord::parse(line)?),
            _ => ()
        }

        Ok(())
    }

    pub fn parse(filepath: &Path) -> Result<Self, ParseError> {
        let f = match File::open(filepath) {
            Ok(file) => file,
            Err(e) => return Err(ParseError::IOError(e)),
        };

        let mut igc_file = Self::_new(filepath);

        for line in BufReader::new(f).lines() {
            let line_result = match line {
                Ok(line) => igc_file._parse_line(&line[..]),
                Err(e) => Err(ParseError::IOError(e)),
            };

            if let Err(e) = line_result {
                return Err(e)
            }
        }

        Ok(igc_file)
    }
}


#[cfg(test)]
mod test {
    use super::{*};

    #[test]
    fn time_parse() {
        assert_eq!(Time::parse("012345").unwrap(),
                   Time { hours: 1, minutes: 23, seconds: 45 });
        assert_eq!(Time::parse("152136").unwrap(),
                   Time { hours: 15, minutes: 21, seconds: 36 });
    }

    #[test]
    fn raw_coord_parse_lat() {
        assert_eq!(RawCoord::parse_lat("5152265N").unwrap(),
                   RawCoord { degrees: 51, minutes: 52, minutes_fraction: 265, sign: Compass::North });
        assert_eq!(RawCoord::parse_lat("5152265S").unwrap(),
                   RawCoord { degrees: 51, minutes: 52, minutes_fraction: 265, sign: Compass::South });
    }

    #[test]
    fn raw_coord_parse_lon() {
        assert_eq!(RawCoord::parse_lon("05152265E").unwrap(),
                   RawCoord { degrees: 51, minutes: 52, minutes_fraction: 265, sign: Compass::East });
        assert_eq!(RawCoord::parse_lon("05152265W").unwrap(),
                   RawCoord { degrees: 51, minutes: 52, minutes_fraction: 265, sign: Compass::West });
    }

    #[test]
    fn simple_brecord_parse() {
        // Only mandatory fields, no optional fields defined in I records.
        let sample_string = "B0941145152265N00032642WA00115";
        let parsed_record = BRecord::parse(sample_string).unwrap();
        let expected = BRecord {
            timestamp: Time { hours: 9, minutes: 41, seconds: 14 },
            lat: RawCoord { degrees: 51, minutes: 52, minutes_fraction: 265, sign: Compass::North },
            lon: RawCoord { degrees: 0, minutes: 32, minutes_fraction: 642, sign: Compass::West },
            fix_valid: FixValid::Valid,
            pressure_alt: 115,
        };

        // Assert the fields individually first, to give better error messages if they don't match
        assert_eq!(parsed_record.timestamp, expected.timestamp);
        assert_eq!(parsed_record.lat, expected.lat);
        assert_eq!(parsed_record.lon, expected.lon);
        assert_eq!(parsed_record.fix_valid, expected.fix_valid);
        assert_eq!(parsed_record.pressure_alt, expected.pressure_alt);
        assert_eq!(parsed_record, expected);
    }
}
