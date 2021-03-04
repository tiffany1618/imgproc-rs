#![allow(dead_code)]

mod common;

use common::setup;
use imgproc_rs::{morphology, colorspace};
use imgproc_rs::image::Image;
use imgproc_rs::io::write;

use std::time::SystemTime;
use imgproc_rs::enums::Bilateral;

const PATH: &str = "images/j.png";

// #[test]
fn erode_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = morphology::erode(&colorspace::rgb_to_grayscale(&img), 3).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/morphology/erode.png").unwrap();
}

// #[test]
fn dilate_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = morphology::dilate(&colorspace::rgb_to_grayscale(&img), 3).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/morphology/dilate.png").unwrap();
}

// #[test]
fn majority_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = morphology::majority(&colorspace::rgb_to_grayscale(&img), 3).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/morphology/majority.png").unwrap();
}

// #[test]
fn open_test() {
    let img = setup("images/j_open.png").unwrap();

    let now = SystemTime::now();
    let proc = morphology::open(&colorspace::rgb_to_grayscale(&img), 3).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/morphology/open.png").unwrap();
}

// #[test]
fn close_test() {
    let img = setup("images/j_close.png").unwrap();

    let now = SystemTime::now();
    let proc = morphology::close(&colorspace::rgb_to_grayscale(&img), 3).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/morphology/close.png").unwrap();
}

// #[test]
fn gradient_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = morphology::gradient(&colorspace::rgb_to_grayscale(&img), 3).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/morphology/gradient.png").unwrap();
}