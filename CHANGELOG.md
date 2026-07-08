# Changelog

All notable changes to this crate are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0]

The version published on crates.io had fallen well behind this repository. This release
brings crates.io up to date and adds whole-file parsing.

### Added

- `parse_records` for parsing an entire in-memory IGC file at once, yielding one `Record`
  per non-blank line via the new `Records` iterator ([#29]).
- `LineError`, which pairs a `ParseError` with the 1-indexed source line it occurred on.
- `parse::read_to_string`, a small helper for reading an `impl Read` into an owned `String`
  to hand to `parse_records`.

### Changed

- Continuous integration moved from Travis CI to GitHub Actions.
- Added `keywords` and `categories` to `Cargo.toml` for crates.io discoverability.

### Previously unpublished

These landed in the repository after the last crates.io release but were never published:

- Optional `serde` support behind the `serde` feature.
- `ParseError` now implements `std::error::Error` (via `thiserror`).
- `Clone` for all record types and `Copy` for the coordinate/time utility types.
- Minimum Supported Rust Version raised to 1.38.0.

[0.3.0]: https://github.com/Joey9801/igc-rs
[#29]: https://github.com/Joey9801/igc-rs/issues/29
