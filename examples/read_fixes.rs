use igc::records::Record;

fn main() {
    let filename = "examples/example.igc";

    // Read the whole file into memory, then parse every record in one pass.
    let contents = std::fs::read_to_string(filename).unwrap();

    for result in igc::parse_records(&contents) {
        let record = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(-1);
            }
        };

        if let Record::B(b_rec) = record {
            println!("b_rec = {:?}", b_rec);
        }
    }
}
