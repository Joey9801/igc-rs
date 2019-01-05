//! This crate provides a minimal, fast parser for IGC files.
//!
//! The low level record parser mirrors the raw format of an IGC file closely, and works to
//! minimize the number of heap allocations made during parsing.
//! It is intended to be used as an unopinionated base for building higher level data structures
//! representing traces/tasks/etc..

pub mod records;
pub mod util;
