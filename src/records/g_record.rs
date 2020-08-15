#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::util::ParseError;

/// A security record.
///
/// The contents of the record are vendor dependent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GRecord<'a> {
    pub data: &'a str,
}

impl<'a> GRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert_eq!(line.as_bytes()[0], b'G');

        Ok(Self { data: &line[1..] })
    }
}

impl<'a> fmt::Display for GRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "G{}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in "G\\PC*") {
            GRecord::parse(&s);
        }
    }
}
