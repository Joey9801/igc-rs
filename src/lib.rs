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

/// Represents a single Gregorian calendar day
#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    /// Parses a date string of the form "DDMMYY"
    /// There are not enough digits for the year in this format (bytes are expensive, yo), so
    /// unilaterlly assume that the date is in the 21st century.
    fn parse(date_string: &str) -> Result<Self, ParseError> {
        assert_eq!(date_string.len(), 6);

        let day = date_string[0..2].parse::<u8>()?;
        let month = date_string[2..4].parse::<u8>()?;
        let year = date_string[4..6].parse::<u16>()? + 2000;

        Ok(Date { day, month, year })
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

#[derive(Debug, PartialEq, Eq)]
pub struct RawPosition {
    pub lat: RawCoord,
    pub lon: RawCoord,
}

impl RawPosition {
    fn parse_lat_lon(pos_string: &str) -> Result<Self, ParseError> {
        assert_eq!(pos_string.len(), 17);
        let lat = RawCoord::parse_lat(&pos_string[0..8])?;
        let lon = RawCoord::parse_lon(&pos_string[8..17])?;

        Ok(Self { lat, lon })
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
    /// assert_eq!(record.timestamp, Time { hours: 9, minutes: 41, seconds: 14 });
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

#[derive(Debug, PartialEq, Eq)]
pub struct CRecordDeclaration {
    pub date: Date,
    pub time: Time,
    pub flight_date: Date,
    pub task_id: u16,
    pub turnpoint_count: u8,
    pub name: Option<String>,
}

impl CRecordDeclaration {
    fn parse(line: &str) -> Result<Self, ParseError> {
        assert!(line.len() >= 25);
        assert!(line.as_bytes()[0] == b'C');

        let date = Date::parse(&line[1..7])?;
        let time = Time::parse(&line[7..13])?;
        let flight_date = Date::parse(&line[13..19])?;
        let task_id = line[19..23].parse::<u16>()?;
        let turnpoint_count = line[23..25].parse::<u8>()?;
        let name = if line.len() > 25 {
            Some(String::from(&line[25..]))
        } else {
            None
        };

        Ok(CRecordDeclaration { date, time, flight_date, task_id, turnpoint_count, name })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CRecordTurnpoint {
    pub position: RawPosition,
    pub name: Option<String>,
}

impl CRecordTurnpoint {
    fn parse(line: &str) -> Result<Self, ParseError> {
        assert!(line.len() >= 18);
        assert!(line.as_bytes()[0] == b'C');

        let position = RawPosition::parse_lat_lon(&line[1..18])?;
        let name = if line.len() > 18 {
            Some(String::from(&line[18..]))
        } else {
            None
        };

        Ok(CRecordTurnpoint { position, name })
    }
}

pub struct Task {
    pub declaration: CRecordDeclaration,
    pub turnpoints: Vec<CRecordTurnpoint>,
}

impl Task {
    fn from(declaration: CRecordDeclaration) -> Self {
        Task { declaration, turnpoints: Vec::<CRecordTurnpoint>::new() }
    }
}


/// Closely represents a parsed IGC file, with minimal post-processing
pub struct IGCFile {
    pub filepath: PathBuf,
    pub fixes: Vec<BRecord>,
    pub task: Option<Task>,
}

impl IGCFile {
    fn _new(filepath: &Path) -> Self {
        IGCFile {
            filepath: filepath.to_path_buf(),
            fixes: Vec::<BRecord>::new(),
            task: None
        }
    }

    fn _parse_line(&mut self, line: &str) -> Result<(), ParseError> {
        match line.as_bytes()[0] {
            b'B' => self.fixes.push(BRecord::parse(line)?),
            b'C' => {
                if let Some(ref mut task) = self.task {
                    task.turnpoints.push(CRecordTurnpoint::parse(line)?);
                } else {
                    self.task = Some(Task::from(CRecordDeclaration::parse(line)?));
                }
            },
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
    fn date_parse() {
        assert_eq!(Date::parse("010118").unwrap(),
                    Date { day: 1, month: 1, year: 2018 });
        assert_eq!(Date::parse("120757").unwrap(),
                    Date { day: 12, month: 7, year: 2057 });
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
            pos: RawPosition {
                lat: RawCoord { degrees: 51, minutes: 52, minutes_fraction: 265, sign: Compass::North },
                lon: RawCoord { degrees: 0, minutes: 32, minutes_fraction: 642, sign: Compass::West },
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

    #[test]
    fn c_record_declaration_parse() {
        let sample_string = "C230718092044000000000204Foo task";
        let parsed_declaration = CRecordDeclaration::parse(sample_string).unwrap();
        let mut expected = CRecordDeclaration {
            date: Date { day: 23, month: 07, year: 2018 },
            time: Time { hours: 09, minutes: 20, seconds: 44 },
            flight_date: Date { day: 00, month: 00, year: 2000 },
            task_id: 2,
            turnpoint_count: 4,
            name: Some("Foo task".to_string())
        };
        assert_eq!(parsed_declaration, expected);

        let sample_string = "C230718092044000000000204";
        let parsed_declaration = CRecordDeclaration::parse(sample_string).unwrap();
        expected.name = None;
        assert_eq!(parsed_declaration, expected);

    }

    #[test]
    fn c_record_turnpoint_parse() {
        let sample_string = "C5156040N00038120WLBZ-Leighton Buzzard NE";
        let parsed_turnpoint = CRecordTurnpoint::parse(sample_string).unwrap();
        let expected = CRecordTurnpoint {
            position: RawPosition {
                lat: RawCoord { degrees: 51, minutes: 56, minutes_fraction: 40, sign: Compass::North },
                lon: RawCoord { degrees: 00, minutes: 38, minutes_fraction: 120, sign: Compass::West },
            },
            name: Some("LBZ-Leighton Buzzard NE".to_string()),
        };

        assert_eq!(parsed_turnpoint, expected);

    }
}
