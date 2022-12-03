use criterion::{criterion_group, criterion_main, Criterion};
use adventofcode::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "day 1",
        |b| {
            b.iter(|| day1::solve())
        }
    );

    c.bench_function(
        "day 2",
        |b| {
            b.iter(|| day2::solve())
        }
    );

    c.bench_function(
        "day 3",
        |b| {
            b.iter(|| day3::solve())
        }
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);