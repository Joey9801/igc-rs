use crate::util::parse_error::ParseError;

// The A Record has to be the first record in an FVU Data File.
// The flight verification unit identification record specifies the unique number of the equipment
// which recorded the flight. This is most likely the manufacturer's serial number.
// Format of the A Record:
//   A M N N N N N T E X T S T R I N G CR LF
//
//  Description   Size      Element      Remarks
//  Manufacturer  1 bytes   M            Valid characters alphanumeric
//  Unique ID     5 bytes   NNNNN        Valid characters alphanumeric
//  ID extension  ? bytes   TEXT STRING  Valid characters alphanumeric

/// Enumeration of all FVU manufacturers defined in the IGC specification v1.00
#[derive(Debug, PartialEq, Eq)]
pub enum Manufacturer {
    Borgelt,
    Cambridge,
    EW,
    Filser,
    Ilec,
    Metron,
    Peschges,
    SkyForce,
    PathTracker,
    Varcom,
    Westerboer,
    Zander,
    Collins,
    Honeywell,
    King,
    Garmin,
    Trimble,
    Motorola,
    Magellan,
    Rockwell,
    Unknown(u8),
}

impl Manufacturer {
    fn parse_code(code: u8) -> Manufacturer {
        use self::Manufacturer::{*};
        match code {
            b'B' => Borgelt,
            b'C' => Cambridge,
            b'E' => EW,
            b'F' => Filser,
            b'I' => Ilec,
            b'M' => Metron,
            b'P' => Peschges,
            b'S' => SkyForce,
            b'T' => PathTracker,
            b'V' => Varcom,
            b'W' => Westerboer,
            b'Z' => Zander,
            b'1' => Collins,
            b'2' => Honeywell,
            b'3' => King,
            b'4' => Garmin,
            b'5' => Trimble,
            b'6' => Motorola,
            b'7' => Magellan,
            b'8' => Rockwell,
            unknown_code => Unknown(unknown_code),
        }
    }
}


/// Represents a single parsed IGC file A Record.
#[derive(Debug, PartialEq, Eq)]
pub struct ARecord<'a> {
    pub manufacturer: Manufacturer,
    pub unique_id: &'a str,
    pub id_extension: Option<&'a str>,
}

impl<'a> ARecord<'a> {
    /// Parse an IGC A Record string
    ///
    /// ```
    /// # extern crate igc_rs;
    /// # use igc_rs::records::{ ARecord, Manufacturer };
    /// let record = ARecord::parse("ACWhizzASDF").unwrap();
    /// assert_eq!(record.manufacturer, Manufacturer::Cambridge);
    /// assert_eq!(record.unique_id, "Whizz");
    /// assert_eq!(record.id_extension, Some("ASDF"));
    /// ```
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert!(line.len() >= 7);
        assert_eq!(&line[0..1], "A");

        let manufacturer = Manufacturer::parse_code(line.as_bytes()[1]);
        let unique_id = &line[2..7];
        let id_extension = if line.len() > 7 {
            Some(&line[7..])
        } else {
            None
        };

        Ok(ARecord { manufacturer, unique_id, id_extension })
    }
}

#[cfg(test)]
mod tests {
    use super::{ARecord,Manufacturer};

    #[test]
    fn arecord_parse() {
        assert_eq!(
            ARecord::parse("ACIdStrFoo").unwrap(),
            ARecord {
                manufacturer: Manufacturer::Cambridge,
                unique_id: "IdStr",
                id_extension: Some("Foo")
            }
        );
    }
}
