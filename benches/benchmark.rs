use clmul::clmul;
use clmul::morton::morton_encode;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn clmul_bench(c: &mut Criterion) {
    c.bench_function("clmul", |b| b.iter(|| clmul(black_box(20), black_box(20))));
}

pub fn morton_clmul_bench(c: &mut Criterion) {
    c.bench_function("morton clmul", |b| {
        b.iter(|| morton_encode(black_box(20), black_box(20)))
    });
}

pub fn morton_magic_bench(c: &mut Criterion) {
    use clmul::morton::morton_encode_magic;
    c.bench_function("morton magic", |b| {
        b.iter(|| morton_encode_magic(black_box(20), black_box(20)))
    });
}

pub fn morton_encode_bench(c: &mut Criterion) {
    use morton_encoding::morton_encode;
    c.bench_function("morton-encoding crate", |b| {
        b.iter(|| morton_encode(black_box([20u32, 20])))
    });
}

pub fn morton_bench(c: &mut Criterion) {
    //note this is actually doing u16,u16 -> u32 the other benchmarks are u32,u32->
    c.bench_function("morton crate", |b| {
        b.iter(|| morton::interleave_morton(black_box(20u16), black_box(20u16)))
    });
}

pub fn zorder_bench(c: &mut Criterion) {
    c.bench_function("zorder crate", |b| {
        b.iter(|| zorder::index_of(black_box([20u16, 20u16])))
    });
}

//
// pub fn morton_index_bench(c: &mut Criterion) {
//     use morton_index::*;
//     use morton_index::dimensions::QuadrantOrdering;
//     //grid depth 5 is because log2(32) = 5
//     c.bench_function("morton_index crate", |b| b.iter(|| StaticMortonIndex2D128::from_grid_index(Vector2::new(black_box(20u32),black_box(20u32)),5,QuadrantOrdering::XY)));
// }

criterion_group!(
    benches,
    clmul_bench,
    morton_clmul_bench,
    morton_magic_bench,
    morton_encode_bench,
    morton_bench,
    zorder_bench
);
// criterion_group!(benches, clmul_bench,morton_clmul_bench,morton_encode_bench,morton_bench,zorder_bench,morton_index_bench);
criterion_main!(benches);
