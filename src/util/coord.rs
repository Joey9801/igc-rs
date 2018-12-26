use super::parse_error::*;

/// Enumeration of cardinal directions
#[derive(Debug, Eq, PartialEq)]
pub enum Compass {
    North,
    South,
    East,
    West,
}

/// Represents a latitude OR longitude, closely representing the form used in IGC files.
#[derive(Debug, PartialEq, Eq)]
pub struct RawCoord {
    pub degrees: u8,              // in range (0, 90) for lat, (0, 180) for lon
    pub minute_thousandths: u16,  // in range (0, 60000). UINT16_MAX = 65535.
    pub sign: Compass,
}

impl RawCoord {
    /// Parse a latitude string of the form "DDMMMMMS"
    pub fn parse_lat(lat_string: &str) -> Result<Self, ParseError> {
        assert_eq!(lat_string.len(), 8);

        let degrees = lat_string[0..2].parse::<u8>()?;
        let minute_thousandths = lat_string[2..7].parse::<u16>()?;
        let sign = match &lat_string[7..8] {
            "N" => Compass::North,
            "S" => Compass::South,
            _ => return Err(ParseError::SyntaxError),
        };

        if degrees > 90 || minute_thousandths > 60000 {
            Err(ParseError::NumberOutOfRange)
        } else {
            Ok(RawCoord { degrees, minute_thousandths, sign })
        }
    }

    /// Parse a longitude string of the form "DDDMMMMMW"
    pub fn parse_lon(lat_string: &str) -> Result<Self, ParseError> {
        assert_eq!(lat_string.len(), 9);

        let degrees = lat_string[0..3].parse::<u8>()?;
        let minute_thousandths = lat_string[3..8].parse::<u16>()?;
        let sign = match &lat_string[8..9] {
            "E" => Compass::East,
            "W" => Compass::West,
            _ => return Err(ParseError::SyntaxError),
        };

        if degrees > 180 || minute_thousandths > 60000 {
            Err(ParseError::NumberOutOfRange)
        } else {
            Ok(RawCoord { degrees, minute_thousandths, sign })
        }
    }
}

/// A raw lat/lon pair.
#[derive(Debug, PartialEq, Eq)]
pub struct RawPosition {
    pub lat: RawCoord,
    pub lon: RawCoord,
}

impl RawPosition {
    pub fn parse_lat_lon(pos_string: &str) -> Result<Self, ParseError> {
        assert_eq!(pos_string.len(), 17);
        let lat = RawCoord::parse_lat(&pos_string[0..8])?;
        let lon = RawCoord::parse_lon(&pos_string[8..17])?;

        Ok(Self { lat, lon })
    }
}


#[cfg(test)]
mod test {
    use super::{RawCoord,Compass};

    #[test]
    fn raw_coord_parse_lat() {
        assert_eq!(RawCoord::parse_lat("5152265N").unwrap(),
                   RawCoord { degrees: 51, minute_thousandths: 52265, sign: Compass::North });
        assert_eq!(RawCoord::parse_lat("5152265S").unwrap(),
                   RawCoord { degrees: 51, minute_thousandths: 52265, sign: Compass::South });
    }

    #[test]
    fn raw_coord_parse_lon() {
        assert_eq!(RawCoord::parse_lon("05152265E").unwrap(),
                   RawCoord { degrees: 51, minute_thousandths: 52265, sign: Compass::East });
        assert_eq!(RawCoord::parse_lon("05152265W").unwrap(),
                   RawCoord { degrees: 51, minute_thousandths: 52265, sign: Compass::West });
    }
}
