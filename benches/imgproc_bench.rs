mod common;
use crate::common::setup;

use imgproc_rs::{colorspace, tone};
use imgproc_rs::enums::Tone;

use criterion::{criterion_group, criterion_main, Criterion};

// pub fn bench_brightness(c: &mut Criterion) {
//     let img = setup("images/tux.png").unwrap();
//
//     c.bench_function("brightness rgb", |b| b.iter(||
//         tone::brightness_rgb_norm(&img, 20)));
//     c.bench_function("brightness rgb avx2", |b| b.iter(||
//         tone::brightness(&img, 20, Tone::Rgb)));
// }

pub fn bench_rgb_to_grayscale(c: &mut Criterion) {
    let img = setup("images/tux.png").unwrap();

    c.bench_function("avx2", |b| b.iter(||
        colorspace::rgb_to_grayscale(&img)));
    c.bench_function("norm", |b| b.iter(||
        colorspace::rgb_to_grayscale_norm(&img)));
}

criterion_group!(benches, bench_rgb_to_grayscale);
criterion_main!(benches);