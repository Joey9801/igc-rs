#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
