use crate::util::parse_error::ParseError;

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
    pub fn parse(time_string: &str) -> Result<Self, ParseError> {
        assert_eq!(time_string.len(), 6);

        let hours = time_string[0..2].parse::<u8>()?;
        let minutes = time_string[2..4].parse::<u8>()?;
        let seconds = time_string[4..6].parse::<u8>()?;

        if hours > 24 || minutes > 60 || seconds > 60 {
            Err(ParseError::NumberOutOfRange)
        } else {
            Ok(Time { hours, minutes, seconds })
        }
    }

    /// Helper method to create a Time from a (hour, minute, second) triplet.
    pub fn from_hms(hours: u8, minutes: u8, seconds: u8) -> Time {
        assert!(hours <= 24);
        assert!(minutes <= 60);
        assert!(seconds <= 60);

        Time { hours, minutes, seconds }
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
    pub fn parse(date_string: &str) -> Result<Self, ParseError> {
        assert_eq!(date_string.len(), 6);

        let day = date_string[0..2].parse::<u8>()?;
        let month = date_string[2..4].parse::<u8>()?;
        let year = date_string[4..6].parse::<u16>()? + 2000;

        if day > 31 || month > 12 {
            Err(ParseError::NumberOutOfRange)
        } else {
            Ok(Date { day, month, year })
        }
    }

    /// Helper method to create a Date from a (day, month, year) triplet
    pub fn from_dmy(day: u8, month: u8, year: u16) -> Date {
        Date { day, month, year }
    }
}


#[cfg(test)]
mod test {
    use super::{Date, Time};

    #[test]
    fn time_parse() {
        assert_eq!(Time::parse("012345").unwrap(),
                   Time::from_hms(1, 23, 45));
        assert_eq!(Time::parse("152136").unwrap(),
                   Time::from_hms(15, 21, 36));
    }

    #[test]
    fn date_parse() {
        assert_eq!(Date::parse("010118").unwrap(),
                   Date::from_dmy(1, 1, 2018));
        assert_eq!(Date::parse("120757").unwrap(),
                   Date::from_dmy(12, 7, 2057));
    }
}
