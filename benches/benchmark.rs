use std::hint::black_box;

use clmul::clmul;
use clmul::morton::morton_encode;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn clmul_bench(c: &mut Criterion) {
    c.bench_function("clmul", |b| b.iter(|| clmul(black_box(20), black_box(20))));
}

pub fn morton_clmul_bench(c: &mut Criterion) {
    c.bench_function("morton u64*2 using clmul", |b| {
        b.iter(|| morton_encode(black_box(20), black_box(20)))
    });
}

pub fn morton_magic_bench(c: &mut Criterion) {
    use clmul::morton::morton_encode_magic;
    c.bench_function("morton u64x2 using this_crate magic numbers", |b| {
        b.iter(|| morton_encode_magic(black_box(20), black_box(20)))
    });
}

pub fn morton_encode_bench(c: &mut Criterion) {
    use morton_encoding::morton_encode;
    c.bench_function("morton-encoding u32x2 crate", |b| {
        b.iter(|| morton_encode(black_box([20u32, 20])))
    });
}

pub fn morton_bench(c: &mut Criterion) {
    //note this is actually doing u16,u16 -> u32 the other benchmarks are u32,u32->
    c.bench_function("morton u16x2 crate", |b| {
        b.iter(|| morton::interleave_morton(black_box(20u16), black_box(20u16)))
    });
}

pub fn zorder_u16_bench(c: &mut Criterion) {
    c.bench_function("zorder u16x2 crate", |b| {
        b.iter(|| zorder::index_of(black_box([20u16, 20u16])))
    });
}

pub fn zorder_u32_bench(c: &mut Criterion) {
    c.bench_function("zorder u32x2 crate", |b| {
        b.iter(|| zorder::index_of(black_box([20u32, 20u32])))
    });
}

pub fn dilate_u32_bench(c: &mut Criterion) {
    c.bench_function("dilate u32x2 crate", |b| {
        b.iter(|| dilate_morton(black_box(20u32), black_box(20u32)))
    });
}

//let location = Morton::<Expand<u16, 3>, 3>::from_coords([1, 2, 3]);
pub fn insides_u32_bench(c: &mut Criterion) {
    use insides::*;
    c.bench_function("insides u32x2 crate", |b| {
        b.iter(|| Morton::<Expand<u32, 2>, 2>::from_coords(black_box([20u32, 20u32])).index())
    });
}

#[inline]
pub fn dilate_morton(x: u32, y: u32) -> u64 {
    use dilate::DilateExpand;

    let xd = x.dilate_expand::<2>().value();
    let yd = y.dilate_expand::<2>().value();
    (yd << 1) | xd
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
    zorder_u16_bench,
    zorder_u32_bench,
    dilate_u32_bench,
    insides_u32_bench
);
// criterion_group!(benches, clmul_bench,morton_clmul_bench,morton_encode_bench,morton_bench,zorder_bench,morton_index_bench);
criterion_main!(benches);
