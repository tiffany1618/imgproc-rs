mod common;
use crate::common::setup;

use imgproc_rs::tone;
use imgproc_rs::enums::Tone;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_brightness(c: &mut Criterion) {
    let img = setup("images/scaled.png").unwrap();

    c.bench_function("brightness rgb", |b| b.iter(||
        tone::brightness_norm(&img, 20)));
    c.bench_function("brightness rgb avx2", |b| b.iter(||
        tone::brightness_256(&img, 20)));
}

criterion_group!(benches, bench_brightness);
criterion_main!(benches);