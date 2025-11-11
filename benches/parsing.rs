use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gullwing::Parser;

fn bench_parse_simple_pattern(c: &mut Criterion) {
    c.bench_function("parse_simple_pattern", |b| {
        let parser = Parser::new("Hello, {name}!").unwrap();
        let text = "Hello, World!";

        b.iter(|| parser.parse(black_box(text)))
    });
}

fn bench_parse_number(c: &mut Criterion) {
    c.bench_function("parse_number", |b| {
        let parser = Parser::new("Value: {value:d}").unwrap();
        let text = "Value: 42";

        b.iter(|| parser.parse(black_box(text)))
    });
}

fn bench_parse_float(c: &mut Criterion) {
    c.bench_function("parse_float", |b| {
        let parser = Parser::new("Pi: {value:f}").unwrap();
        let text = "Pi: 3.14159";

        b.iter(|| parser.parse(black_box(text)))
    });
}

fn bench_parse_with_grouping(c: &mut Criterion) {
    c.bench_function("parse_with_grouping", |b| {
        let parser = Parser::new("{value:,}").unwrap();
        let text = "1,234,567";

        b.iter(|| parser.parse(black_box(text)))
    });
}

fn bench_parse_multiple_fields(c: &mut Criterion) {
    c.bench_function("parse_multiple_fields", |b| {
        let parser = Parser::new("{name}: {value:d} items at {price:.2f} each").unwrap();
        let text = "Product: 10 items at 9.99 each";

        b.iter(|| parser.parse(black_box(text)))
    });
}

fn bench_parse_search(c: &mut Criterion) {
    c.bench_function("parse_search", |b| {
        let parser = Parser::new("Error {code:d}: {msg}").unwrap();
        let text = "This is a log file with Error 404: Not found in the middle";

        b.iter(|| parser.search(black_box(text)))
    });
}

fn bench_parse_findall(c: &mut Criterion) {
    c.bench_function("parse_findall", |b| {
        let parser = Parser::new("{key}={value}").unwrap();
        let text = "config: name=test, port=8080, host=localhost, debug=true";

        b.iter(|| parser.findall(black_box(text)))
    });
}

fn bench_parse_complex_pattern(c: &mut Criterion) {
    c.bench_function("parse_complex_pattern", |b| {
        let parser = Parser::new(
            "Name: {name:<20} | Amount: {amount:>10,.2f} | ID: {id:#06x}"
        ).unwrap();
        let text = "Name: Alice                | Amount:   1,234.56 | ID: 0x002a";

        b.iter(|| parser.parse(black_box(text)))
    });
}

fn bench_parse_hex_number(c: &mut Criterion) {
    c.bench_function("parse_hex_number", |b| {
        let parser = Parser::new("Address: {addr:x}").unwrap();
        let text = "Address: deadbeef";

        b.iter(|| parser.parse(black_box(text)))
    });
}

fn bench_parse_pattern_creation(c: &mut Criterion) {
    c.bench_function("parse_pattern_creation", |b| {
        b.iter(|| {
            Parser::new(black_box("Name: {name} | Value: {value:d}"))
        })
    });
}

criterion_group!(
    benches,
    bench_parse_simple_pattern,
    bench_parse_number,
    bench_parse_float,
    bench_parse_with_grouping,
    bench_parse_multiple_fields,
    bench_parse_search,
    bench_parse_findall,
    bench_parse_complex_pattern,
    bench_parse_hex_number,
    bench_parse_pattern_creation
);
criterion_main!(benches);
