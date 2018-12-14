use super::parse_error::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Compass {
    North,
    South,
    East,
    West,
}

/// Represents a raw coordinate, as it appears in an IGC file.
#[derive(Debug, PartialEq, Eq)]
pub struct RawCoord {
    pub degrees: u8,           // in range (0, 90) for lat, (0, 180) for lon
    pub minutes: u8,           // in range (0, 60)
    pub minutes_fraction: u16, // Thousandths of a minute, in range(0, 1000)
    pub sign: Compass,
}

impl RawCoord {
    /// Parse a latitude string of the form "DDMMMMMS"
    pub fn parse_lat(lat_string: &str) -> Result<Self, ParseError> {
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
    pub fn parse_lon(lat_string: &str) -> Result<Self, ParseError> {
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
}
