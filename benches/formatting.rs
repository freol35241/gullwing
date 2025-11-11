use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gullwing::{Formatter, Value};
use std::collections::HashMap;

fn bench_format_string_simple(c: &mut Criterion) {
    c.bench_function("format_string_simple", |b| {
        let formatter = Formatter::new("Hello, {name}!").unwrap();
        let mut values = HashMap::new();
        values.insert("name".to_string(), Value::Str("World".to_string()));

        b.iter(|| formatter.format_map(black_box(&values)))
    });
}

fn bench_format_number_simple(c: &mut Criterion) {
    c.bench_function("format_number_simple", |b| {
        let formatter = Formatter::new("Value: {value}").unwrap();
        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::Int(42));

        b.iter(|| formatter.format_map(black_box(&values)))
    });
}

fn bench_format_number_with_grouping(c: &mut Criterion) {
    c.bench_function("format_number_with_grouping", |b| {
        let formatter = Formatter::new("{value:,}").unwrap();
        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::Int(1234567890));

        b.iter(|| formatter.format_map(black_box(&values)))
    });
}

fn bench_format_float_precision(c: &mut Criterion) {
    c.bench_function("format_float_precision", |b| {
        let formatter = Formatter::new("{value:.2f}").unwrap();
        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::Float(3.14159265));

        b.iter(|| formatter.format_map(black_box(&values)))
    });
}

fn bench_format_aligned_padded(c: &mut Criterion) {
    c.bench_function("format_aligned_padded", |b| {
        let formatter = Formatter::new("{value:*>20}").unwrap();
        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::Str("test".to_string()));

        b.iter(|| formatter.format_map(black_box(&values)))
    });
}

fn bench_format_hex_with_prefix(c: &mut Criterion) {
    c.bench_function("format_hex_with_prefix", |b| {
        let formatter = Formatter::new("{value:#x}").unwrap();
        let mut values = HashMap::new();
        values.insert("value".to_string(), Value::Int(255));

        b.iter(|| formatter.format_map(black_box(&values)))
    });
}

fn bench_format_complex_pattern(c: &mut Criterion) {
    c.bench_function("format_complex_pattern", |b| {
        let formatter =
            Formatter::new("Name: {name:<20} | Amount: {amount:>10,.2f} | ID: {id:#06x}").unwrap();
        let mut values = HashMap::new();
        values.insert("name".to_string(), Value::Str("Alice".to_string()));
        values.insert("amount".to_string(), Value::Float(1234.56));
        values.insert("id".to_string(), Value::Int(42));

        b.iter(|| formatter.format_map(black_box(&values)))
    });
}

fn bench_format_multiple_fields(c: &mut Criterion) {
    c.bench_function("format_multiple_fields", |b| {
        let formatter = Formatter::new("{a} {b} {c} {d} {e} {f} {g} {h} {i} {j}").unwrap();
        let mut values = HashMap::new();
        for (i, ch) in "abcdefghij".chars().enumerate() {
            values.insert(ch.to_string(), Value::Int(i as i64));
        }

        b.iter(|| formatter.format_map(black_box(&values)))
    });
}

criterion_group!(
    benches,
    bench_format_string_simple,
    bench_format_number_simple,
    bench_format_number_with_grouping,
    bench_format_float_precision,
    bench_format_aligned_padded,
    bench_format_hex_with_prefix,
    bench_format_complex_pattern,
    bench_format_multiple_fields
);
criterion_main!(benches);
