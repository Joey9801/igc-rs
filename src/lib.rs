//! This crate provides a minimal, fast parser for IGC files.
//!
//! The low level record parser mirrors the raw format of an IGC file closely, and works to
//! minimize the number of heap allocations made during parsing.
//! It is intended to be used as an unopinionated base for building higher level data structures
//! representing traces/tasks/etc..
//!
//! Individual lines can be parsed with [`records::Record::parse_line`]. To parse an entire
//! in-memory file at once, use [`parse_records`], which yields one [`records::Record`] per
//! line.

#[cfg(test)]
#[macro_use]
extern crate proptest;

pub mod parse;
pub mod records;
pub mod util;

pub use crate::parse::{LineError, Records, parse_records};
