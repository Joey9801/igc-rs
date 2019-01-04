use std::fs::File;
use memmap::MmapOptions;
use igc::records::Record;

fn main() {
    let filename = "examples/example.igc";

    let file = File::open(filename).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file) }.unwrap();

    for line in std::str::from_utf8(&mmap).unwrap().lines() {
        let record = match Record::parse_line(line) {
            Ok(record) => record,
            Err(_) => std::process::exit(-1),
        };

        if let Record::B(b_rec) = record {
            println!("b_rec = {:?}", b_rec);
        }
    }
}
