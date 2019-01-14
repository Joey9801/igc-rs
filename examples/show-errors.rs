use std::env;
use std::fs;
use std::path;
use std::io::{BufRead,BufReader};

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
        let file = fs::File::open(path.clone()).unwrap();

        for (i, result) in BufReader::new(file).lines().enumerate() {
            let line_number = i + 1;

            let line = match result {
                Err(error) => {
                    println!("{}:{} ERROR {}", filename, line_number, error);
                    continue;
                },
                Ok(line) => line,
            };

            match Record::parse_line(&line) {
                Err(error) => {
                    println!("{}:{} ERROR {:?}: {}", filename, line_number, error, line);
                    continue;
                },
                Ok(_) => {},
            };
        }
    }
}

fn is_igc_file(path: &path::PathBuf) -> bool {
    match path.extension() {
        None => false,
        Some(os_str) => {
            match os_str.to_str() {
                Some("igc") => true,
                _ => false,
            }
        }
    }
}
