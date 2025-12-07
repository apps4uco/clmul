use std::hint::black_box;

use clmul::clmul;
use criterion::{Criterion, criterion_group, criterion_main};

pub fn clmul_bench(c: &mut Criterion) {
    c.bench_function("clmul", |b| b.iter(|| clmul(black_box(20), black_box(20))));
}
criterion_group!(benches, clmul_bench,);
criterion_main!(benches);
