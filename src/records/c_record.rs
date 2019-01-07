use crate::util::{Date, ParseError, RawPosition, Time};

/// The first flavor of C Record - a task record which defines some properties of the whole task.
///
/// The IGC specification states that a conforming file containing a task declaration will contain
/// a CRecordDeclaration, immediately followed (turnpoint_count + 4) CRecordTurnpoints's.
/// The extra 4 turnpoints are for the takeoff/land locations, and the task start/finish locations
#[derive(Debug, PartialEq, Eq)]
pub struct CRecordDeclaration<'a> {
    pub date: Date,
    pub time: Time,
    pub flight_date: Date,
    pub task_id: u16,
    pub turnpoint_count: u8,
    pub task_name: Option<&'a str>,
}

impl<'a> CRecordDeclaration<'a> {
    /// Parse a string as a C record task declaration
    ///
    /// ```
    /// # use igc::{ records::CRecordDeclaration, util::{Time,Date} };
    /// let record = CRecordDeclaration::parse("C230718092044000000000204Foo task").unwrap();
    /// assert_eq!(record.date, Date::from_dmy(23, 7, 18));
    /// assert_eq!(record.time, Time::from_hms(9, 20, 44));
    /// assert_eq!(record.task_id, 2);
    /// assert_eq!(record.turnpoint_count, 4);
    /// assert_eq!(record.task_name, Some("Foo task"));
    /// ```
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        if line.len() < 25 {
            return Err(ParseError::SyntaxError);
        }

        assert!(line.as_bytes()[0] == b'C');

        let date = line[1..7].parse()?;
        let time = line[7..13].parse()?;
        let flight_date = line[13..19].parse()?;
        let task_id = line[19..23].parse::<u16>()?;
        let turnpoint_count = line[23..25].parse::<u8>()?;
        let task_name = if line.len() > 25 {
            Some(&line[25..])
        } else {
            None
        };

        Ok(CRecordDeclaration {
            date,
            time,
            flight_date,
            task_id,
            turnpoint_count,
            task_name,
        })
    }
}

/// The second flavor of C Record - a start / turn / end point for a task.
#[derive(Debug, PartialEq, Eq)]
pub struct CRecordTurnpoint<'a> {
    pub position: RawPosition,
    pub turnpoint_name: Option<&'a str>,
}

impl<'a> CRecordTurnpoint<'a> {
    /// Parse a string as a C record task turnpoint
    ///
    /// ```
    /// # use igc::{ records::CRecordTurnpoint, util::{Compass,RawLatitude,RawLongitude} };
    /// let record = CRecordTurnpoint::parse("C5156040N00038120WLBZ-Leighton Buzzard NE").unwrap();
    /// assert_eq!(record.position.lat, RawLatitude::new(51, 56_040, Compass::North));
    /// assert_eq!(record.position.lon, RawLongitude::new(0, 38_120, Compass::West));
    /// assert_eq!(record.turnpoint_name, Some("LBZ-Leighton Buzzard NE"));
    /// ```
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert!(line.len() >= 18);
        assert!(line.as_bytes()[0] == b'C');

        let position = line[1..18].parse()?;
        let turnpoint_name = if line.len() > 18 {
            Some(&line[18..])
        } else {
            None
        };

        Ok(CRecordTurnpoint {
            position,
            turnpoint_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{Compass, RawLatitude, RawLongitude, RawPosition};

    #[test]
    fn c_record_declaration_parse() {
        let sample_string = "C230718092044000000000204Foo task";
        let parsed_declaration = CRecordDeclaration::parse(sample_string).unwrap();
        let mut expected = CRecordDeclaration {
            date: Date {
                day: 23,
                month: 07,
                year: 18,
            },
            time: Time {
                hours: 09,
                minutes: 20,
                seconds: 44,
            },
            flight_date: Date {
                day: 00,
                month: 00,
                year: 00,
            },
            task_id: 2,
            turnpoint_count: 4,
            task_name: Some("Foo task"),
        };
        assert_eq!(parsed_declaration, expected);

        let sample_string = "C230718092044000000000204";
        let parsed_declaration = CRecordDeclaration::parse(sample_string).unwrap();
        expected.task_name = None;
        assert_eq!(parsed_declaration, expected);
    }

    #[test]
    fn c_record_turnpoint_parse() {
        let sample_string = "C5156040N00038120WLBZ-Leighton Buzzard NE";
        let parsed_turnpoint = CRecordTurnpoint::parse(sample_string).unwrap();
        let expected = CRecordTurnpoint {
            position: RawPosition {
                lat: RawLatitude::new(51, 56_040, Compass::North),
                lon: RawLongitude::new(0, 38_120, Compass::West),
            },
            turnpoint_name: Some("LBZ-Leighton Buzzard NE"),
        };

        assert_eq!(parsed_turnpoint, expected);
    }
}
