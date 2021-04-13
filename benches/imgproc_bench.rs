mod common;
use crate::common::setup;

use imgproc_rs::tone;
use imgproc_rs::enums::Tone;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_brightness(c: &mut Criterion) {
    let img = setup("images/beach.jpg").unwrap();

    c.bench_function("brightness rgb", |b| b.iter(||
        tone::brightness(&img, 20, Tone::Rgb)));
}

criterion_group!(benches, bench_brightness);
criterion_main!(benches);