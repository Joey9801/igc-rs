extern crate igc_rs;
extern crate memmap;

use std::fs::File;

use memmap::MmapOptions;

fn main() {
    let filename = "examples/example.igc";

    let file = File::open(filename).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file) }.unwrap();
    let igc_file = igc_rs::IgcFile::parse_bytes(mmap.as_ref()).unwrap();

    for record in igc_file.records {
        if let igc_rs::Record::B(b_rec) = record {
            println!("b_rec = {:?}", b_rec);
        }
    }
}
