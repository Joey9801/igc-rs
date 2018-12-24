extern crate igc_rs;
extern crate memmap;

use igc_rs::{IgcFile, records::Record};

use std::fs::File;

use memmap::MmapOptions;

fn main() {
    let filename = "examples/example.igc";

    let file = File::open(filename).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file) }.unwrap();
    let igc_file = IgcFile::parse_bytes(mmap.as_ref()).unwrap();

    for record in igc_file.records {
        if let Record::B(b_rec) = record {
            println!("b_rec = {:?}", b_rec);
        }
    }
}
