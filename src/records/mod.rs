//! Low level record parsing API
//!
//! ```
//! # extern crate igc;
//! use igc::records::{Record,DataSource};
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
mod k_record;
mod l_record;

pub use self::a_record::*;
pub use self::b_record::BRecord;
pub use self::c_record::{CRecordDeclaration, CRecordTurnpoint};
pub use self::d_record::DRecord;
pub use self::e_record::ERecord;
pub use self::extension::{Extendable, Extension, IRecord, JRecord};
pub use self::f_record::FRecord;
pub use self::g_record::GRecord;
pub use self::h_record::{DataSource, HRecord};
pub use self::k_record::KRecord;
pub use self::l_record::LRecord;

/// Sum type of all possible records in an IGC file.
#[derive(Debug)]
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
    /// use igc::records::{Record,DataSource};
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
        let rec = match line.as_bytes()[0] {
            b'A' => Record::A(ARecord::parse(line)?),
            b'B' => Record::B(BRecord::parse(line)?),
            b'C' => {
                // In a turnpoint C record, the 9th character is the N/S of the latitutde
                // In a declaration type C record, it is a number (part of the declaration time)
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

#[cfg(test)]
mod tests {
    #[test]
    fn parse_record_line() {
        let rec = super::Record::parse_line("ACIdStrFoo").unwrap();
        println!("rec = {:?}", rec);
        if let super::Record::A(a_record) = rec {
            println!("The record was an A record: {:?}", a_record);
        } else {
            println!("The record was not an A record :( {:?}", rec);
        }
    }
}
