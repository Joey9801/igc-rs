use std::fmt;
use std::str::FromStr;

use super::parse_error::*;

/// Enumeration of cardinal directions
#[derive(Debug, Eq, PartialEq)]
pub enum Compass {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Compass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letter = match self {
            Compass::North => 'N',
            Compass::South => 'S',
            Compass::East => 'E',
            Compass::West => 'W',
        };

        write!(f, "{}", letter)
    }
}

/// Represents a latitude OR longitude, closely representing the form used in IGC files.
#[derive(Debug, PartialEq, Eq)]
pub struct RawCoord {
    pub degrees: u8,             // in range (0, 90) for lat, (0, 180) for lon
    pub minute_thousandths: u16, // in range (0, 60000). UINT16_MAX = 65535.
    pub sign: Compass,
}

impl From<RawCoord> for f32 {
    fn from(coord: RawCoord) -> Self {
        let value = Self::from(coord.degrees) + Self::from(coord.minute_thousandths) / 60_000.;
        match coord.sign {
            Compass::North | Compass::East => value,
            Compass::South | Compass::West => -value,
        }
    }
}

impl From<RawCoord> for f64 {
    fn from(coord: RawCoord) -> Self {
        let value = Self::from(coord.degrees) + Self::from(coord.minute_thousandths) / 60_000.;
        match coord.sign {
            Compass::North | Compass::East => value,
            Compass::South | Compass::West => -value,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawLatitude(pub RawCoord);

impl RawLatitude {
    pub fn new(degrees: u8, minute_thousandths: u16, sign: Compass) -> Self {
        assert!(degrees <= 90);
        assert!(minute_thousandths < 60_000);
        assert!(sign == Compass::North || sign == Compass::South);

        Self(RawCoord {
            degrees,
            minute_thousandths,
            sign,
        })
    }
}

impl FromStr for RawLatitude {
    type Err = ParseError;

    /// Parse a latitude string of the form "DDMMMMMS"
    fn from_str(lat_string: &str) -> Result<Self, ParseError> {
        assert_eq!(
            lat_string.len(),
            8,
            "Raw latitude strings are 8 characters long"
        );

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
            Ok(Self(RawCoord {
                degrees,
                minute_thousandths,
                sign,
            }))
        }
    }
}

impl fmt::Display for RawLatitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}{:05}{}",
            self.0.degrees, self.0.minute_thousandths, self.0.sign
        )
    }
}

impl From<RawLatitude> for f32 {
    fn from(lat: RawLatitude) -> Self {
        lat.0.into()
    }
}

impl From<RawLatitude> for f64 {
    fn from(lat: RawLatitude) -> Self {
        lat.0.into()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawLongitude(pub RawCoord);

impl RawLongitude {
    pub fn new(degrees: u8, minute_thousandths: u16, sign: Compass) -> Self {
        assert!(degrees <= 180);
        assert!(minute_thousandths < 60_000);
        assert!(sign == Compass::East || sign == Compass::West);

        Self(RawCoord {
            degrees,
            minute_thousandths,
            sign,
        })
    }
}

impl FromStr for RawLongitude {
    type Err = ParseError;

    /// Parse a longitude string of the form "DDDMMMMMW"
    fn from_str(lon_string: &str) -> Result<Self, ParseError> {
        assert_eq!(
            lon_string.len(),
            9,
            "Raw longitude strings are 9 characters long"
        );

        let degrees = lon_string[0..3].parse::<u8>()?;
        let minute_thousandths = lon_string[3..8].parse::<u16>()?;
        let sign = match &lon_string[8..9] {
            "E" => Compass::East,
            "W" => Compass::West,
            _ => return Err(ParseError::SyntaxError),
        };

        if degrees > 180 || minute_thousandths > 60000 {
            Err(ParseError::NumberOutOfRange)
        } else {
            Ok(Self(RawCoord {
                degrees,
                minute_thousandths,
                sign,
            }))
        }
    }
}

impl fmt::Display for RawLongitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:03}{:05}{}",
            self.0.degrees, self.0.minute_thousandths, self.0.sign
        )
    }
}

impl From<RawLongitude> for f32 {
    fn from(lon: RawLongitude) -> Self {
        lon.0.into()
    }
}

impl From<RawLongitude> for f64 {
    fn from(lon: RawLongitude) -> Self {
        lon.0.into()
    }
}

/// A raw lat/lon pair.
#[derive(Debug, PartialEq, Eq)]
pub struct RawPosition {
    pub lat: RawLatitude,
    pub lon: RawLongitude,
}

impl FromStr for RawPosition {
    type Err = ParseError;

    fn from_str(pos_string: &str) -> Result<Self, ParseError> {
        assert_eq!(pos_string.len(), 17);
        let lat = pos_string[0..8].parse()?;
        let lon = pos_string[8..17].parse()?;

        Ok(Self { lat, lon })
    }
}

impl fmt::Display for RawPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.lat, self.lon)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn raw_lat_parse() {
        assert_eq!(
            RawLatitude::new(51, 52_265, Compass::North),
            "5152265N".parse().unwrap()
        );

        assert_eq!(
            RawLatitude::new(51, 52_265, Compass::South),
            "5152265S".parse().unwrap()
        );
    }

    #[test]
    fn raw_coord_parse_lon() {
        assert_eq!(
            RawLongitude::new(51, 52_265, Compass::East),
            "05152265E".parse().unwrap()
        );

        assert_eq!(
            RawLongitude::new(51, 52_265, Compass::West),
            "05152265W".parse().unwrap()
        );
    }

    #[test]
    fn raw_lat_format() {
        assert_eq!(
            format!("{}", RawLatitude::new(51, 23_355, Compass::North)),
            "5123355N"
        );
        assert_eq!(
            format!("{}", RawLatitude::new(51, 23_355, Compass::South)),
            "5123355S"
        );
    }

    #[test]
    fn raw_lon_format() {
        assert_eq!(
            format!("{}", RawLongitude::new(51, 23_355, Compass::East)),
            "05123355E"
        );
        assert_eq!(
            format!("{}", RawLongitude::new(51, 23_355, Compass::West)),
            "05123355W"
        );
    }

    #[test]
    fn parse_raw_position() {
        assert_eq!(
            "5152265N05152265W".parse::<RawPosition>().unwrap(),
            RawPosition {
                lat: RawLatitude::new(51, 52_265, Compass::North),
                lon: RawLongitude::new(51, 52_265, Compass::West)
            }
        );
    }

    #[test]
    fn convert_to_float() {
        assert_relative_eq!(
            "05152265E".parse::<RawLongitude>().unwrap().into(),
            51.871082f32
        );
        assert_relative_eq!(
            "5152265S".parse::<RawLatitude>().unwrap().into(),
            -51.87108333333333f64
        );
    }
}
