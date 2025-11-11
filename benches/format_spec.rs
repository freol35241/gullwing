use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gullwing::FormatSpec;

fn bench_parse_simple_spec(c: &mut Criterion) {
    c.bench_function("parse_simple_spec", |b| {
        b.iter(|| FormatSpec::parse(black_box(">10")))
    });
}

fn bench_parse_complex_spec(c: &mut Criterion) {
    c.bench_function("parse_complex_spec", |b| {
        b.iter(|| FormatSpec::parse(black_box("0<+#20,.2f")))
    });
}

fn bench_parse_all_features_spec(c: &mut Criterion) {
    c.bench_function("parse_all_features_spec", |b| {
        b.iter(|| FormatSpec::parse(black_box("*>+z#030_,.6e")))
    });
}

criterion_group!(
    benches,
    bench_parse_simple_spec,
    bench_parse_complex_spec,
    bench_parse_all_features_spec
);
criterion_main!(benches);
