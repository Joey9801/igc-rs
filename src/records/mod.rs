//! Low level record parsing API
//!
//! ```
//! # extern crate igc;
//! use igc::records::{DataSource, Record};
//! match Record::parse_line("HFFTYFRTYPE:LXNAV,LX8000F") {
//!     Ok(Record::H(header_rec)) => {
//!         assert_eq!(header_rec.data_source, DataSource::FVU);
//!         assert_eq!(header_rec.mnemonic, "FTY");
//!         assert_eq!(header_rec.friendly_name, Some("FRTYPE"));
//!         assert_eq!(header_rec.data, "LXNAV,LX8000F");
//!     }
//!     _ => unreachable!(),
//! }
//! ```

use std::fmt;

use crate::util::ParseError;

mod a_record;
mod b_record;
mod c_record;
mod d_record;
mod e_record;
mod extension;
mod f_record;
mod g_record;
mod h_record;
mod i_record;
mod j_record;
mod k_record;
mod l_record;

pub use self::a_record::*;
pub use self::b_record::BRecord;
pub use self::c_record::{CRecordDeclaration, CRecordTurnpoint};
pub use self::d_record::DRecord;
pub use self::e_record::ERecord;
pub use self::extension::{Extendable, Extension};
pub use self::f_record::FRecord;
pub use self::g_record::GRecord;
pub use self::h_record::{DataSource, HRecord};
pub use self::i_record::IRecord;
pub use self::j_record::JRecord;
pub use self::k_record::KRecord;
pub use self::l_record::LRecord;

/// Sum type of all possible records in an IGC file.
#[derive(Debug, PartialEq, Eq)]
pub enum Record<'a> {
    A(ARecord<'a>),
    B(BRecord<'a>),
    CDeclaration(CRecordDeclaration<'a>),
    CTurnpoint(CRecordTurnpoint<'a>),
    D(DRecord<'a>),
    E(ERecord<'a>),
    F(FRecord<'a>),
    G(GRecord<'a>),
    H(HRecord<'a>),
    I(IRecord<'a>),
    J(JRecord<'a>),
    K(KRecord<'a>),
    L(LRecord<'a>),

    /// Wildcard record type, containing the string that wasn't recognized.
    Unrecognised(&'a str),
}

impl<'a> Record<'a> {
    /// Perform a minimal parsing of a single IGC file line.
    ///
    /// ```
    /// use igc::records::{DataSource, Record};
    /// match Record::parse_line("HFFTYFRTYPE:LXNAV,LX8000F") {
    ///     Ok(Record::H(header_rec)) => {
    ///         assert_eq!(header_rec.data_source, DataSource::FVU);
    ///         assert_eq!(header_rec.mnemonic, "FTY");
    ///         assert_eq!(header_rec.friendly_name, Some("FRTYPE"));
    ///         assert_eq!(header_rec.data, "LXNAV,LX8000F");
    ///     }
    ///     _ => unreachable!(),
    /// }
    /// ```
    pub fn parse_line(line: &'a str) -> Result<Self, ParseError> {
        if line.is_empty() {
            return Err(ParseError::SyntaxError);
        }

        let rec = match line.as_bytes()[0] {
            b'A' => Record::A(ARecord::parse(line)?),
            b'B' => Record::B(BRecord::parse(line)?),
            b'C' => {
                // In a turnpoint C record, the 9th character is the N/S of the latitutde
                // In a declaration type C record, it is a number (part of the declaration time)
                if line.len() < 9 {
                    return Err(ParseError::SyntaxError);
                }

                match line.as_bytes()[8] {
                    b'N' | b'S' => Record::CTurnpoint(CRecordTurnpoint::parse(line)?),
                    _ => Record::CDeclaration(CRecordDeclaration::parse(line)?),
                }
            }
            b'D' => Record::D(DRecord::parse(line)?),
            b'E' => Record::E(ERecord::parse(line)?),
            b'F' => Record::F(FRecord::parse(line)?),
            b'G' => Record::G(GRecord::parse(line)?),
            b'H' => Record::H(HRecord::parse(line)?),
            b'I' => Record::I(IRecord::parse(line)?),
            b'J' => Record::J(JRecord::parse(line)?),
            b'K' => Record::K(KRecord::parse(line)?),
            b'L' => Record::L(LRecord::parse(line)?),
            _ => Record::Unrecognised(line),
        };

        Ok(rec)
    }
}

impl<'a> fmt::Display for Record<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Record::*;
        // This is horrible...
        match self {
            A(rec) => write!(f, "{}", rec),
            B(rec) => write!(f, "{}", rec),
            CDeclaration(rec) => write!(f, "{}", rec),
            CTurnpoint(rec) => write!(f, "{}", rec),
            D(rec) => write!(f, "{}", rec),
            E(rec) => write!(f, "{}", rec),
            F(rec) => write!(f, "{}", rec),
            G(rec) => write!(f, "{}", rec),
            H(rec) => write!(f, "{}", rec),
            I(rec) => write!(f, "{}", rec),
            J(rec) => write!(f, "{}", rec),
            K(rec) => write!(f, "{}", rec),
            L(rec) => write!(f, "{}", rec),
            Unrecognised(line) => write!(f, "{}", line),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_string() {
        assert!(Record::parse_line("").is_err());
    }

    #[test]
    fn record_parse_line() {
        let rec = Record::parse_line("ACAMWatFoo").unwrap();
        assert_eq!(
            rec,
            Record::A(ARecord {
                manufacturer: Manufacturer::CambridgeAeroInstruments,
                unique_id: "Wat",
                id_extension: Some("Foo")
            })
        );
    }

    #[test]
    fn record_parse_short_c_record() {
        assert!(Record::parse_line("C123").is_err());
    }

    #[test]
    fn record_format() {
        let expected_str = "ACAMWatFoo";
        let rec = Record::A(ARecord {
            manufacturer: Manufacturer::CambridgeAeroInstruments,
            unique_id: "Wat",
            id_extension: Some("Foo"),
        });

        assert_eq!(format!("{}", rec), expected_str);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in "\\PC*") {
            Record::parse_line(&s);
        }
    }
}
