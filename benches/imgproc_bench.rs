mod common;
use crate::common::setup;

use imgproc_rs::{colorspace, tone};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_brightness(c: &mut Criterion) {
    let img = setup("images/tux.png").unwrap();

    c.bench_function("brightness", |b| b.iter(||
        tone::brightness(&img, 20)));
}

pub fn bench_rgb_to_grayscale(c: &mut Criterion) {
    let img = setup("images/tux.png").unwrap();

    c.bench_function("grayscale", |b| b.iter(||
        colorspace::rgb_to_grayscale(&img)));
}

pub fn bench_rgb_to_hsv(c: &mut Criterion) {
    let img = setup("images/spectrum.jpg").unwrap();

    c.bench_function("u8", |b| b.iter(||
        colorspace::rgb_to_hsv(&img)));
    c.bench_function("f32", |b| b.iter(||
        colorspace::rgb_to_hsv_f32(&img)));
}

pub fn bench_saturation(c: &mut Criterion) {
    let img = setup("images/spectrum.jpg").unwrap();

    c.bench_function("saturation", |b| b.iter(||
        tone::saturation(&img, 100)));
}

criterion_group!(benches, bench_saturation);
criterion_main!(benches);