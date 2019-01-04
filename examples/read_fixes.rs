extern crate igc;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use igc::records::Record;

fn main() {
    let filename = "examples/example.igc";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    for result in reader.lines() {
        let line = match result {
            Ok(line) => line,
            Err(_) => std::process::exit(-1),
        };

        let record = match Record::parse_line(&line) {
            Ok(record) => record,
            Err(_) => std::process::exit(-1),
        };

        if let Record::B(b_rec) = record {
            println!("b_rec = {:?}", b_rec);
        }
    }
}
