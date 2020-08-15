#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::util::parse_error::ParseError;

/// Represents a specific time of day with second precision.
///
/// Does not contain any timezone information as the IGC specification mandates UTC everywhere.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Time {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
}

impl Time {
    /// Parse a time string of the form "HHMMSS"
    pub fn parse(time_string: &str) -> Result<Self, ParseError> {
        assert_eq!(time_string.len(), 6);

        if !time_string.is_ascii() {
            return Err(ParseError::NonASCIICharacters);
        }

        let hours = time_string[0..2].parse::<u8>()?;
        let minutes = time_string[2..4].parse::<u8>()?;
        let seconds = time_string[4..6].parse::<u8>()?;

        if hours > 24 || minutes > 60 || seconds > 60 {
            Err(ParseError::NumberOutOfRange)
        } else {
            Ok(Time {
                hours,
                minutes,
                seconds,
            })
        }
    }

    /// Helper method to create a Time from a (hour, minute, second) triplet.
    pub fn from_hms(hours: u8, minutes: u8, seconds: u8) -> Time {
        assert!(hours <= 24);
        assert!(minutes <= 60);
        assert!(seconds <= 60);

        Time {
            hours,
            minutes,
            seconds,
        }
    }

    /// Helper method to get the seconds since midnight.
    pub fn seconds_since_midnight(&self) -> u32 {
        let mins: u32 = u32::from(self.hours) * 60 + u32::from(self.minutes);
        mins * 60 + u32::from(self.seconds)
    }
}

impl FromStr for Time {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        Self::parse(s)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}{:02}{:02}", self.hours, self.minutes, self.seconds)
    }
}

/// Represents a single Gregorian calendar day
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Date {
    /// In the range [1, 31]
    pub day: u8,

    /// In the range [1, 12]
    pub month: u8,

    /// Only the least significant two digits of the year. In the range [0, 99]
    pub year: u8,
}

impl Date {
    /// Parses a date string of the form "DDMMYY"
    /// There are not enough digits for the year in this format (bytes are expensive, yo).
    pub fn parse(date_string: &str) -> Result<Self, ParseError> {
        assert_eq!(date_string.len(), 6);

        if !date_string.is_ascii() {
            return Err(ParseError::NonASCIICharacters);
        }

        let day = date_string[0..2].parse::<u8>()?;
        let month = date_string[2..4].parse::<u8>()?;
        let year = date_string[4..6].parse::<u8>()?;

        if day > 31 || month > 12 {
            Err(ParseError::NumberOutOfRange)
        } else {
            Ok(Date { day, month, year })
        }
    }

    /// Helper method to create a Date from a (day, month, year) triplet
    pub fn from_dmy(day: u8, month: u8, year: u8) -> Date {
        assert!(day >= 1 && day <= 31);
        assert!(month >= 1 && month <= 12);
        assert!(year <= 99);
        Date { day, month, year }
    }
}

impl FromStr for Date {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        Self::parse(s)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}{:02}{:02}", self.day, self.month, self.year)
    }
}

#[cfg(test)]
mod test {
    use super::{Date, Time};

    #[test]
    fn time_parse() {
        assert_eq!("012345".parse::<Time>().unwrap(), Time::from_hms(1, 23, 45));
        assert_eq!(
            "152136".parse::<Time>().unwrap(),
            Time::from_hms(15, 21, 36)
        );
    }

    #[test]
    fn time_parse_with_invalid_char_boundary() {
        assert!(Time::parse("🌀aa").is_err());
    }

    #[test]
    fn time_fmt() {
        assert_eq!(format!("{}", Time::from_hms(1, 23, 45)), "012345");
    }

    #[test]
    fn time_seconds_since_midnight() {
        assert_eq!(
            Time {
                hours: 0,
                minutes: 0,
                seconds: 0
            }
            .seconds_since_midnight(),
            0
        );
        assert_eq!(
            Time {
                hours: 1,
                minutes: 2,
                seconds: 3
            }
            .seconds_since_midnight(),
            3600 + 120 + 3
        );
    }

    #[test]
    fn date_parse() {
        assert_eq!("010118".parse::<Date>().unwrap(), Date::from_dmy(1, 1, 18));
        assert_eq!("120757".parse::<Date>().unwrap(), Date::from_dmy(12, 7, 57));
    }

    #[test]
    fn date_parse_with_invalid_char_boundary() {
        assert!(Date::parse("🌀aa").is_err());
    }

    #[test]
    fn date_fmt() {
        assert_eq!(format!("{}", Date::from_dmy(5, 10, 18)), "051018");
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn time_parse_back_to_original(h in 0u8..24, m in 0u8..60, s in 0u8..60) {
            let time = Time::from_hms(h, m, s);
            prop_assert_eq!(Time::parse(&format!("{}", time)).unwrap(), time);
        }

        #[test]
        #[allow(unused_must_use)]
        fn date_parse_back_to_original(d in 1u8..32, m in 1u8..13, y in 0u8..100) {
            let date = Date::from_dmy(d, m, y);
            prop_assert_eq!(Date::parse(&format!("{}", date)).unwrap(), date);
        }
    }
}
