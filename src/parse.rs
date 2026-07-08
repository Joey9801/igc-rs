//! Whole-file parsing API.
//!
//! The low-level [`Record::parse_line`](crate::records::Record::parse_line) API parses a
//! single line at a time. This module builds on top of it to parse an entire in-memory IGC
//! file with a single call, yielding one [`Record`] per line.
//!
//! Because every parsed [`Record`] borrows from its source line, the caller must own the
//! backing buffer (an owned `String` or a `&str` that outlives the iterator). Reading a
//! whole file into memory first is therefore unavoidable — [`read_to_string`] is provided
//! as a small convenience for pulling an `impl Read` into an owned `String`.
//!
//! ```
//! use igc::records::Record;
//!
//! let file = "\
//! ALXVK4AFLIGHT:1
//! HFDTE230718
//! B1101355206343N00006198WA0058700558";
//!
//! for result in igc::parse_records(file) {
//!     match result {
//!         Ok(Record::B(fix)) => assert_eq!(fix.pressure_alt, 587),
//!         Ok(_) => {}
//!         Err(e) => panic!("line {}: {}", e.line_number, e.error),
//!     }
//! }
//! ```

use std::error::Error;
use std::fmt;
use std::io::{self, Read};
use std::str::Lines;

use crate::records::Record;
use crate::util::ParseError;

/// A [`ParseError`] together with the 1-indexed source line on which it occurred.
///
/// The line number counts every physical line of the input, including any blank lines that
/// [`Records`] itself skips, so it always points at the offending line in the original file.
#[derive(Debug)]
pub struct LineError {
    /// The 1-indexed line number the error occurred on.
    pub line_number: usize,
    /// The underlying parse error.
    pub error: ParseError,
}

impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}: {}", self.line_number, self.error)
    }
}

impl Error for LineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

/// An iterator yielding one parsed [`Record`] per non-blank line of an IGC file.
///
/// Created by [`parse_records`]. Borrows the input `&'a str`; the caller owns the backing
/// buffer. Blank lines (including any trailing newline) are skipped. Any line that fails to
/// parse is surfaced as an [`Err`] containing a [`LineError`]; iteration is not halted, so
/// callers may either stop on the first error or keep going.
pub struct Records<'a> {
    lines: Lines<'a>,
    line_number: usize,
}

impl<'a> Iterator for Records<'a> {
    type Item = Result<Record<'a>, LineError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip fully-blank lines, advancing the physical line counter for each so that any
        // reported line number matches the original file. Content lines are never trimmed:
        // the record parsers slice by byte offset (e.g. B/K extension fields), so trimming
        // would corrupt them.
        loop {
            let line = self.lines.next()?;
            self.line_number += 1;

            if line.trim().is_empty() {
                continue;
            }

            return Some(Record::parse_line(line).map_err(|error| LineError {
                line_number: self.line_number,
                error,
            }));
        }
    }
}

/// Parse every record in an already-in-memory IGC file.
///
/// Returns an iterator that borrows `input` and yields one [`Record`] per non-blank line.
/// Blank lines are skipped, and unknown record types are returned as
/// [`Record::Unrecognised`], matching [`Record::parse_line`].
///
/// The caller owns the backing buffer, so this composes naturally with
/// [`std::fs::read_to_string`] or [`read_to_string`]:
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let buf = std::fs::read_to_string("flight.igc")?;
/// let records = igc::parse_records(&buf).collect::<Result<Vec<_>, _>>()?;
/// println!("parsed {} records", records.len());
/// # Ok(())
/// # }
/// ```
pub fn parse_records(input: &str) -> Records<'_> {
    Records {
        lines: input.lines(),
        line_number: 0,
    }
}

/// Read an entire reader into an owned `String` ready to hand to [`parse_records`].
///
/// This is a thin convenience wrapper over [`Read::read_to_string`]. Whole-file parsing must
/// own its buffer, so the streaming benefit of `io::Read` cannot be preserved regardless.
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let file = std::fs::File::open("flight.igc")?;
/// let buf = igc::parse::read_to_string(file)?;
/// let count = igc::parse_records(&buf).filter(Result::is_ok).count();
/// # let _ = count;
/// # Ok(())
/// # }
/// ```
pub fn read_to_string(mut reader: impl Read) -> io::Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../examples/example.igc");

    #[test]
    fn parses_sample_file() {
        // Every line of the sample file is a valid record.
        let records = parse_records(SAMPLE)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let non_blank = SAMPLE.lines().filter(|l| !l.trim().is_empty()).count();
        assert_eq!(records.len(), non_blank);
        assert!(records.iter().any(|r| matches!(r, Record::B(_))));
    }

    #[test]
    fn skips_blank_lines() {
        let input = "\nHFDTE230718\n\n\nHFFXA015\n\n";
        let records = parse_records(input).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn empty_input_yields_no_records() {
        // Contrast with Record::parse_line(""), which returns an error.
        assert_eq!(parse_records("").count(), 0);
        assert!(Record::parse_line("").is_err());
    }

    #[test]
    fn reports_correct_line_number_on_error() {
        // Blank lines are counted so the reported line matches the original file. Line 4
        // ("B123") is too short to be a valid B record.
        let input = "HFDTE230718\n\nHFFXA015\nB123\nHFFXA016";
        let mut iter = parse_records(input);

        assert!(matches!(iter.next(), Some(Ok(Record::H(_)))));
        assert!(matches!(iter.next(), Some(Ok(Record::H(_)))));

        let err = iter.next().unwrap().unwrap_err();
        assert_eq!(err.line_number, 4);

        // Iteration continues past the error.
        assert!(matches!(iter.next(), Some(Ok(Record::H(_)))));
        assert!(iter.next().is_none());
    }

    #[test]
    fn line_error_display_includes_line_number() {
        let err = parse_records("B123").next().unwrap().unwrap_err();
        assert!(format!("{}", err).starts_with("line 1: "));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in "\\PC*") {
            for record in parse_records(&s) {
                let _ = record;
            }
        }
    }
}
