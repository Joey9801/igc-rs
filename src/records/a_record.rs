use std::fmt;

use crate::util::DisplayOption;
use crate::util::ParseError;

#[derive(Debug, PartialEq, Eq)]
pub enum Manufacturer<'a> {
    Aircotec,
    CambridgeAeroInstruments,
    ClearNavInstruments,
    DataSwan,
    EwAvionics,
    Filser,
    Flarm,
    Flytech,
    Garrecht,
    ImiGlidingEquipment,
    Logstream,
    LxNavigation,
    LxNav,
    Naviter,
    NewTechnologies,
    NielsenKellerman,
    Peschges,
    PressFinishElectronics,
    PrintTechnik,
    Scheffel,
    StreamlineDataInstruments,
    TriadisEngineering,
    Zander,
    UnknownSingle(u8),
    UnknownTriple(&'a str),
}

impl<'a> Manufacturer<'a> {
    pub fn parse_single_char(character: u8) -> Self {
        use self::Manufacturer::*;
        match character {
            b'I' => Aircotec,
            b'C' => CambridgeAeroInstruments,
            b'D' => DataSwan,
            b'E' => EwAvionics,
            b'F' => Filser,
            b'G' => Flarm,
            b'A' => Garrecht,
            b'M' => ImiGlidingEquipment,
            b'L' => LxNavigation,
            b'V' => LxNav,
            b'N' => NewTechnologies,
            b'K' => NielsenKellerman,
            b'P' => Peschges,
            b'R' => PrintTechnik,
            b'H' => Scheffel,
            b'S' => StreamlineDataInstruments,
            b'T' => TriadisEngineering,
            b'Z' => Zander,
            unknown => UnknownSingle(unknown),
        }
    }

    pub fn parse_triple_char(triple: &'a str) -> Self {
        use self::Manufacturer::*;
        match triple {
            "ACT" => Aircotec,
            "CAM" => CambridgeAeroInstruments,
            "CNI" => ClearNavInstruments,
            "DSX" => DataSwan,
            "EWA" => EwAvionics,
            "FIL" => Filser,
            "FLA" => Flarm,
            "FLY" => Flytech,
            "GCS" => Garrecht,
            "IMI" => ImiGlidingEquipment,
            "LGS" => Logstream,
            "LXN" => LxNavigation,
            "LXV" => LxNav,
            "NAV" => Naviter,
            "NTE" => NewTechnologies,
            "NKL" => NielsenKellerman,
            "PES" => Peschges,
            "PFE" => PressFinishElectronics,
            "PRT" => PrintTechnik,
            "SCH" => Scheffel,
            "SDI" => StreamlineDataInstruments,
            "TRI" => TriadisEngineering,
            "ZAN" => Zander,
            _ => UnknownTriple(triple),
        }
    }

    pub fn to_single_char(&self) -> Option<u8> {
        use self::Manufacturer::*;
        // It's sad that rustfmt currently nukes the alignment on these match arms
        match self {
            Aircotec => Some(b'I'),
            CambridgeAeroInstruments => Some(b'C'),
            DataSwan => Some(b'D'),
            EwAvionics => Some(b'E'),
            Filser => Some(b'F'),
            Flarm => Some(b'G'),
            Garrecht => Some(b'A'),
            ImiGlidingEquipment => Some(b'M'),
            LxNavigation => Some(b'L'),
            LxNav => Some(b'V'),
            NewTechnologies => Some(b'N'),
            NielsenKellerman => Some(b'K'),
            Peschges => Some(b'P'),
            PrintTechnik => Some(b'R'),
            Scheffel => Some(b'H'),
            StreamlineDataInstruments => Some(b'S'),
            TriadisEngineering => Some(b'T'),
            Zander => Some(b'Z'),
            UnknownSingle(s) => Some(*s),
            _ => None,
        }
    }

    pub fn to_triple_char(&self) -> Option<&'a str> {
        use self::Manufacturer::*;
        match self {
            Aircotec => Some("ACT"),
            CambridgeAeroInstruments => Some("CAM"),
            ClearNavInstruments => Some("CNI"),
            DataSwan => Some("DSX"),
            EwAvionics => Some("EWA"),
            Filser => Some("FIL"),
            Flarm => Some("FLA"),
            Flytech => Some("FLY"),
            Garrecht => Some("GCS"),
            ImiGlidingEquipment => Some("IMI"),
            Logstream => Some("LGS"),
            LxNavigation => Some("LXN"),
            LxNav => Some("LXV"),
            Naviter => Some("NAV"),
            NewTechnologies => Some("NTE"),
            NielsenKellerman => Some("NKL"),
            Peschges => Some("PES"),
            PressFinishElectronics => Some("PFE"),
            PrintTechnik => Some("PRT"),
            Scheffel => Some("SCH"),
            StreamlineDataInstruments => Some("SDI"),
            TriadisEngineering => Some("TRI"),
            Zander => Some("ZAN"),
            UnknownTriple(t) => Some(t),
            _ => None,
        }
    }
}

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
    /// # use igc::records::{ ARecord, Manufacturer };
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
