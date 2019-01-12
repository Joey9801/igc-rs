use criterion::{criterion_group, criterion_main, Criterion};
use igc::records::Record;

fn parse_records(s: &str) -> Vec<Record> {
    s.lines()
        .map(|line| Record::parse_line(line).unwrap())
        .collect::<Vec<_>>()
}

fn criterion_benchmark(c: &mut Criterion) {
    let s = include_str!("../examples/example.igc");

    c.bench_function("parse example.igc", move |b| b.iter(|| parse_records(s)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
