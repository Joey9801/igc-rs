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
            b'n' => Flytech,
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
}

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
    /// # extern crate igc;
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

        let manufacturer = Manufacturer::parse_triple_char(&line[1..4]);
        let unique_id = &line[4..7];
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
            ARecord::parse("ACAMWatFoo").unwrap(),
            ARecord {
                manufacturer: Manufacturer::CambridgeAeroInstruments,
                unique_id: "Wat",
                id_extension: Some("Foo")
            }
        );
    }
}
