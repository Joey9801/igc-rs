use crate::util::parse_error::ParseError;

pub mod a_record;
pub mod b_record;
pub mod c_record;

pub use self::a_record::ARecord;
pub use self::b_record::BRecord;
pub use self::c_record::{CRecordDeclaration,CRecordTurnpoint};

#[derive(Debug)]
pub enum Record<'a> {
    A (ARecord<'a>),
    B (BRecord),
    CDeclaration (CRecordDeclaration<'a>),
    CTurnpoint (CRecordTurnpoint<'a>),
    Unrecognised,
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
            _ => Record::Unrecognised,
        };

        Ok(rec)
    }
}

#[cfg(test)]
mod test {
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
