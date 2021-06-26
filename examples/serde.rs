use igc::records::Record;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[cfg(not(feature = "serde"))]
compile_error!("This example requires igc-rs built with the `serde` feature enabled");

fn main() {
    let filename = "examples/example.igc";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    for result in reader.lines().take(50) {
        let line = match result {
            Ok(line) => line,
            Err(_) => std::process::exit(-1),
        };

        let record = match Record::parse_line(&line) {
            Ok(record) => record,
            Err(_) => std::process::exit(-1),
        };

        println!(
            "{}",
            serde_json::to_string_pretty(&record).expect("Failed to serialize")
        );
    }
}
