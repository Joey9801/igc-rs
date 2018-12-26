use crate::util::parse_error::ParseError;

mod a_record;
mod b_record;
mod c_record;
mod d_record;
mod e_record;
mod f_record;
mod g_record;
mod h_record;
mod extension;
mod k_record;
mod l_record;


pub use self::a_record::*;
pub use self::b_record::BRecord;
pub use self::c_record::{CRecordDeclaration,CRecordTurnpoint};
pub use self::d_record::DRecord;
pub use self::e_record::ERecord;
pub use self::f_record::FRecord;
pub use self::g_record::GRecord;
pub use self::h_record::HRecord;
pub use self::extension::{ExtensionDefRecord,Extension,Extendable};
pub use self::k_record::KRecord;
pub use self::l_record::LRecord;

#[derive(Debug)]
pub enum Record<'a> {
    A (ARecord<'a>),
    B (BRecord<'a>),
    CDeclaration (CRecordDeclaration<'a>),
    CTurnpoint (CRecordTurnpoint<'a>),
    D (DRecord<'a>),
    E (ERecord<'a>),
    F (FRecord<'a>),
    G (GRecord<'a>),
    H (HRecord<'a>),
    I (ExtensionDefRecord<'a>),
    J (ExtensionDefRecord<'a>),
    K (KRecord<'a>),
    L (LRecord<'a>),
    Unrecognised (&'a str),
}

impl<'a> Record<'a> {
    pub fn parse_line(line: &'a str) -> Result<Self, ParseError> {
        let rec = match line.as_bytes()[0] {
            b'A' => Record::A(ARecord::parse(line)?),
            b'B' => Record::B(BRecord::parse(line)?),
            b'C' => {
                // In a turnpoint C record, the 9th character is the N/S of the latitutde
                // In a declaration type C record, it is a number (part of the declaration time)
                let ninth = line.as_bytes()[8];
                if ninth == b'N' || ninth == b'S' {
                    Record::CTurnpoint(CRecordTurnpoint::parse(line)?)
                } else {
                    Record::CDeclaration(CRecordDeclaration::parse(line)?)
                }
            },
            b'D' => Record::D(DRecord::parse(line)?),
            b'E' => Record::E(ERecord::parse(line)?),
            b'F' => Record::F(FRecord::parse(line)?),
            b'G' => Record::G(GRecord::parse(line)?),
            b'H' => Record::H(HRecord::parse(line)?),
            b'I' => Record::I(ExtensionDefRecord::parse(line)?),
            b'J' => Record::J(ExtensionDefRecord::parse(line)?),
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
