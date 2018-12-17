mod util;
pub use self::util::parse_error::ParseError;

mod records;
pub use self::records::*;


/// Closely represents a parsed IGC file, with minimal post-processing
pub struct IgcFile<'a> {
    pub records: Vec<Record<'a>>
}

impl<'a> IgcFile<'a> {
    /// Parse a slice of bytes as a 
    pub fn parse_bytes(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let mut records : Vec<Record> = Vec::new();

        for line in std::str::from_utf8(bytes)?.lines() {
            records.push(Record::parse_line(line)?);
        }

        Ok(IgcFile { records })
    }
}
