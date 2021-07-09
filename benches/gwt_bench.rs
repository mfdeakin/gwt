#![allow(non_snake_case)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::f32;

fn numLen_log10(v: u32) -> usize {
    let v = v as f32;
    return (v.log10().floor() + 1.0) as usize;
}

fn numLen_into_str(v: u32) -> usize {
    return v.to_string().len();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("log10", |b| b.iter(|| numLen_log10(black_box(20))));
    c.bench_function("into_str", |b| b.iter(|| numLen_into_str(black_box(6547))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);