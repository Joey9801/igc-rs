use std::borrow::Cow;
use std::env;
use std::fs;
use std::path;

use encoding::all::{ISO_8859_1, UTF_8};
use encoding::{DecoderTrap, Encoding};

use igc::records::Record;

fn main() {
    // collect command line arguments
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: show-errors DIR");
        std::process::exit(1);
    }

    // iterate over files in the folder
    for entry in fs::read_dir(&args[1]).unwrap() {
        let path: path::PathBuf = entry.unwrap().path();
        if !is_igc_file(&path) {
            continue;
        }

        let filename = path.file_name().unwrap().to_str().unwrap();

        // open file in buffered reader
        let bytes = fs::read(&path).unwrap();
        let text = match as_text(&bytes) {
            Err(error) => {
                println!("{} ERROR {}", filename, error);
                continue;
            }
            Ok(text) => text,
        };

        for (i, line) in text.lines().enumerate() {
            let line_number = i + 1;

            if let Err(error) = Record::parse_line(line) {
                println!("{}:{} ERROR {:?}: {}", filename, line_number, error, line);
                continue;
            };
        }
    }
}

fn is_igc_file(path: &path::Path) -> bool {
    matches!(
        path.extension().and_then(|os_str| os_str.to_str()),
        Some("igc")
    )
}

pub fn as_text(bytes: &[u8]) -> Result<String, Cow<'_, str>> {
    UTF_8
        .decode(bytes, DecoderTrap::Strict)
        .or_else(|_| ISO_8859_1.decode(bytes, DecoderTrap::Strict))
}
