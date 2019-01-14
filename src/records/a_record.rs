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

        Ok(ARecord {
            manufacturer,
            unique_id,
            id_extension,
        })
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
            ARecord {
                manufacturer: Manufacturer::CambridgeAeroInstruments,
                unique_id: "Wat",
                id_extension: Some("Foo")
            }
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
                ARecord {
                    manufacturer: Manufacturer::CambridgeAeroInstruments,
                    unique_id: "Wat",
                    id_extension: Some("Foo")
                }
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
