//! # IGC
//! This is a small crate providing a minimal, fast parser for IGC files.
//! The low level record parser mirrors the raw format of an IGC file closely, and guarantees that
//! no heap allocations will be made during parsing. It is intended to be used as an unopinionated
//! base for building higher level data structures representing flights.

extern crate arrayvec;

pub mod util;
pub mod records;
