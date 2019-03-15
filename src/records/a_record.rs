use std::fmt;

use crate::util::DisplayOption;
use crate::util::Manufacturer;
use crate::util::ParseError;

/// Represents the FVU ID record
#[derive(Debug, PartialEq, Eq)]
pub struct ARecord<'a> {
    pub manufacturer: Manufacturer<'a>,
    pub unique_id: &'a str,
    pub id_extension: Option<&'a str>,
}

impl<'a> ARecord<'a> {
    pub fn new(
        manufacturer: Manufacturer<'a>,
        unique_id: &'a str,
        id_extension: Option<&'a str>,
    ) -> ARecord<'a> {
        ARecord {
            manufacturer,
            unique_id,
            id_extension,
        }
    }

    /// Parse an IGC A Record string
    ///
    /// ```
    /// # use igc::records::ARecord;
    /// # use igc::util::Manufacturer;
    /// let record = ARecord::parse("ACAMWatFoo").unwrap();
    /// assert_eq!(record.manufacturer, Manufacturer::CambridgeAeroInstruments);
    /// assert_eq!(record.unique_id, "Wat");
    /// assert_eq!(record.id_extension, Some("Foo"));
    /// ```
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert_eq!(&line[0..1], "A");

        if line.len() < 7 {
            return Err(ParseError::SyntaxError);
        }

        // check for old spec format (e.g. `AC00069`)
        //
        // all known loggers that use this old format use numeric serial numbers
        // so we assume that to be a sufficient heuristic to detect this format
        if line.bytes().skip(2).take(5).all(|b| b.is_ascii_digit()) {
            let manufacturer_byte = line.as_bytes()[1];
            if !manufacturer_byte.is_ascii() {
                return Err(ParseError::NonASCIICharacters);
            }

            let id_extension = if line.len() > 7 {
                Some(&line[7..])
            } else {
                None
            };

            let manufacturer = Manufacturer::parse_single_char(manufacturer_byte);
            return Ok(ARecord::new(manufacturer, &line[2..7], id_extension));
        }

        // check for old spec format with three-letter manufacturer (e.g. `AFIL01460FLIGHT:1`)
        //
        // all known loggers that use this old format use numeric serial numbers
        // so we assume that to be a sufficient heuristic to detect this format
        if line.len() >= 9 && line.bytes().skip(4).take(5).all(|b| b.is_ascii_digit()) {
            if !line.bytes().take(4).all(|b| b.is_ascii()) {
                return Err(ParseError::NonASCIICharacters);
            }

            let id_extension = if line.len() > 9 {
                Some(&line[9..])
            } else {
                None
            };

            let manufacturer = Manufacturer::parse_triple_char(&line[1..4]);
            return Ok(ARecord::new(manufacturer, &line[4..9], id_extension));
        }

        if !line.bytes().take(7).all(|b| b.is_ascii()) {
            return Err(ParseError::NonASCIICharacters);
        }

        let manufacturer = Manufacturer::parse_triple_char(&line[1..4]);
        let unique_id = &line[4..7];
        let id_extension = if line.len() > 7 {
            Some(&line[7..])
        } else {
            None
        };

        Ok(ARecord::new(manufacturer, unique_id, id_extension))
    }
}

impl<'a> fmt::Display for ARecord<'a> {
    /// Formats this record as it should appear in an IGC file.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A{}{}{}",
            DisplayOption(self.manufacturer.to_triple_char()),
            self.unique_id,
            DisplayOption(self.id_extension)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{ARecord, Manufacturer};

    #[test]
    fn arecord_parse() {
        assert_eq!(
            ARecord::parse("ACAMWatFoo").unwrap(),
            ARecord::new(Manufacturer::CambridgeAeroInstruments, "Wat", Some("Foo"))
        );

        assert_eq!(
            ARecord::parse("ACAMWatFoo").unwrap(),
            ARecord::new(Manufacturer::CambridgeAeroInstruments, "Wat", Some("Foo"))
        );

        // from https://skylines.aero/files/th_46eg6ng1.igc
        assert_eq!(
            ARecord::parse("AFLA6NG").unwrap(),
            ARecord::new(Manufacturer::Flarm, "6NG", None)
        );

        // from http://www.gliding.ch/images/news/lx20/fichiers_igc.htm
        assert_eq!(
            ARecord::parse("AC00069").unwrap(),
            ARecord::new(Manufacturer::CambridgeAeroInstruments, "00069", None)
        );

        // from LX8000 (see `example.igc`)
        assert_eq!(
            ARecord::parse("ALXVK4AFLIGHT:1").unwrap(),
            ARecord::new(Manufacturer::LxNav, "K4A", Some("FLIGHT:1"))
        );

        // from https://github.com/XCSoar/XCSoar/blob/v6.8.11/test/data/lxn_to_igc/18BF14K1.igc
        assert_eq!(
            ARecord::parse("AFIL01460FLIGHT:1").unwrap(),
            ARecord::new(Manufacturer::Filser, "01460", Some("FLIGHT:1"))
        );

        assert_eq!(
            ARecord::parse("AX00000").unwrap(),
            ARecord::new(Manufacturer::UnknownSingle(b'X'), "00000", None)
        );

        assert_eq!(
            ARecord::parse("AXYZABC:foobar").unwrap(),
            ARecord::new(Manufacturer::UnknownTriple("XYZ"), "ABC", Some(":foobar"))
        );

        assert_eq!(
            ARecord::parse("AWIN000").unwrap(),
            ARecord::new(Manufacturer::UnknownTriple("WIN"), "000", None)
        );
    }

    #[test]
    fn parse_with_invalid_char_boundary() {
        assert!(ARecord::parse("A0ꢀ￼").is_err());
    }

    #[test]
    fn arecord_fmt() {
        assert_eq!(
            format!(
                "{}",
                ARecord::new(Manufacturer::CambridgeAeroInstruments, "Wat", Some("Foo"))
            ),
            "ACAMWatFoo"
        );
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in "A\\PC*") {
            ARecord::parse(&s);
        }
    }
}
