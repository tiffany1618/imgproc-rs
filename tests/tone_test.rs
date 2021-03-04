#![allow(dead_code)]

mod common;

use common::setup;
use imgproc_rs::tone;
use imgproc_rs::io::write;

use std::time::SystemTime;
use imgproc_rs::enums::{Tone, White};

const PATH: &str = "images/beach.jpg";

// #[test]
fn brightness_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = tone::brightness(&img, 20, Tone::Rgb).unwrap();
    println!("rgb: {}", now.elapsed().unwrap().as_millis());

    let now = SystemTime::now();
    let proc2 = tone::brightness(&img, 20, Tone::Lab).unwrap();
    println!("xyz: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/tone/bright_rgb.png").unwrap();
    write(&proc2, "images/tests/tone/bright_xyz.png").unwrap();
}

// #[test]
fn contrast_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = tone::contrast(&img, 1.5, Tone::Rgb).unwrap();
    println!("rgb: {}", now.elapsed().unwrap().as_millis());

    let now = SystemTime::now();
    let proc2 = tone::contrast(&img, 1.5, Tone::Lab).unwrap();
    println!("xyz: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/tone/contrast_rgb.png").unwrap();
    write(&proc2, "images/tests/tone/contrast_xyz.png").unwrap();
}

// #[test]
fn saturation_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = tone::saturation(&img, 10).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/tone/saturation.png").unwrap();
}

// #[test]
fn gamma_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = tone::gamma(&img, 1.5, 255).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/tone/gamma.png").unwrap();
}

// #[test]
fn histogram_equalization_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = tone::histogram_equalization(&img, 0.5, &White::D50, 255.0).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/tone/histogram.png").unwrap();
}
