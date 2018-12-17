use crate::util::parse_error::ParseError;

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


#[derive(Debug, PartialEq, Eq)]
pub struct ARecord<'a> {
    pub manufacturer: Manufacturer,
    pub unique_id: &'a str,
    pub id_extension: Option<&'a str>,
}

impl<'a> ARecord<'a> {
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
mod test {
    use super::{ARecord,Manufacturer};

    #[test]
    fn arecord_parse() {
        assert_eq!(
            ARecord::parse("ACIdStrFoo").unwrap(),
            ARecord {
                manufacturer: Manufacturer::Cambridge,
                unique_id: "IdStr".to_string(),
                id_extension: Some("Foo".to_string())
            }
        );
    }
}
