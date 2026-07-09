# igc-rs &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs]][docs.rs]

[Build Status]: https://github.com/Joey9801/igc-rs/actions/workflows/ci.yml/badge.svg
[actions]: https://github.com/Joey9801/igc-rs/actions/workflows/ci.yml
[Latest Version]: https://img.shields.io/crates/v/igc.svg
[crates.io]: https://crates.io/crates/igc
[Docs]: https://docs.rs/igc/badge.svg
[docs.rs]: https://docs.rs/igc

**igc-rs provides a minimal, fast parser for [IGC flight recorder files][igc-spec] in the Rust language.**

[igc-spec]: https://www.fai.org/sites/default/files/igc_fr_specification_2020-11_with_al6.pdf

The parser mirrors the raw record structure of an IGC file closely and works to
minimise the number of heap allocations made while parsing — every parsed record
borrows from the source line rather than copying out of it. It is intended as an
unopinionated base on which to build higher level representations of
traces/tasks/etc.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
igc = "0.3"
```

### Parsing a single line

The low-level entry point is [`Record::parse_line`], which classifies one line
and parses it into the matching record type:

```rust
use igc::records::{Record, DataSource};

match Record::parse_line("HFFTYFRTYPE:LXNAV,LX8000F") {
    Ok(Record::H(header)) => {
        assert_eq!(header.data_source, DataSource::FVU);
        assert_eq!(header.mnemonic, "FTY");
        assert_eq!(header.data, "LXNAV,LX8000F");
    }
    _ => unreachable!(),
}
```

### Parsing a whole file

[`parse_records`] parses an entire in-memory file at once, yielding one
[`Record`] per non-blank line. Because each record borrows from the input, the
caller owns the backing buffer:

```rust
use igc::records::Record;

let file = std::fs::read_to_string("flight.igc")?;

let mut fixes = 0;
for record in igc::parse_records(&file) {
    match record {
        Ok(Record::B(fix)) => fixes += 1,
        Ok(_) => {}
        // Parsing does not stop on the first bad line; the line number is
        // reported so you can decide whether to skip or bail.
        Err(e) => eprintln!("line {}: {}", e.line_number, e.error),
    }
}
println!("{fixes} fixes");
```

See the [`examples/`](examples) directory for runnable programs, including
[`read_fixes.rs`](examples/read_fixes.rs) and
[`show-errors.rs`](examples/show-errors.rs).

## Features

- `serde` *(off by default)* — derives `Serialize`/`Deserialize` for every
  record and utility type, so parsed records can be re-emitted as JSON or any
  other serde format. See [`examples/serde.rs`](examples/serde.rs).

## Minimum Supported Rust Version

The Minimum Supported Rust Version for this crate is **1.71.0**, verified in CI.
Raising it is considered a minor, not a breaking, change.

## License

Licensed under the [MIT license](LICENSE).

[`Record`]: https://docs.rs/igc/latest/igc/records/enum.Record.html
[`Record::parse_line`]: https://docs.rs/igc/latest/igc/records/enum.Record.html#method.parse_line
[`parse_records`]: https://docs.rs/igc/latest/igc/fn.parse_records.html
