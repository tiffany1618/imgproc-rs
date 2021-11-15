#![allow(dead_code)]

mod common;

use common::setup;
use imgproc_rs::{colorspace, convert};
use imgproc_rs::image::Image;
use imgproc_rs::io::write;

use std::time::SystemTime;
use imgproc_rs::enums::White;

const PATH: &str = "images/spectrum.jpg";

// #[test]
fn rgb_to_grayscale_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let gray = colorspace::rgb_to_grayscale(&img);
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&gray, "images/tests/colorspace/gray.png").unwrap();
}

// #[test]
fn rgb_to_grayscale_f64_test() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let now = SystemTime::now();
    let gray = colorspace::rgb_to_grayscale_f64(&img);
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&gray.into(), "images/tests/colorspace/gray_f64.png").unwrap();
}

// #[test]
fn srgb_to_xyz_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = colorspace::srgb_to_xyz_f32(&img);
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&convert::scale_channels(&proc, 0.0, 0.0, 1.0, 255.0).unwrap().into(), "images/tests/colorspace/srgb_xyz.png").unwrap();
}

// #[test]
fn xyz_to_srgb_test() {
    let img: Image<f64> = setup("images/tests/colorspace/srgb_xyz.png").unwrap().into();

    let now = SystemTime::now();
    let proc = colorspace::xyz_to_srgb_f32(&convert::scale_channels(&img, 0.0, 0.0, 255.0, 1.0).unwrap());
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/colorspace/xyz_srgb.png").unwrap();
}

// #[test]
fn lab_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let lab = colorspace::srgb_to_lab_f32(&img, &White::D50);
    println!("lab: {}", now.elapsed().unwrap().as_millis());

    let now = SystemTime::now();
    let proc = colorspace::lab_to_srgb_f32(&lab, &White::D50);
    println!("rgb: {}", now.elapsed().unwrap().as_millis());

    // for c in 0..(proc.info().channels as usize) {
    //     let mut max = -255.0;
    //     let mut min = 255.0;
    //
    //     for i in 0..(proc.info().size() as usize) {
    //         if proc[i][c] < min {
    //             min = proc[i][c];
    //         }
    //         if proc[i][c] > max {
    //             max = proc[i][c];
    //         }
    //     }
    //     println!("{}: {}, {}", c, min, max);
    // }

    write(&proc, "images/tests/colorspace/lab_srgb.png").unwrap();
}

// #[test]
fn rgb_to_hsv_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = colorspace::rgb_to_hsv_f32(&img);
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&convert::scale_channels(&proc, 0.0, 0.0, 1.0, 255.0).unwrap().into(), "images/tests/colorspace/rgb_hsv.png").unwrap();
}

// #[test]
fn hsv_to_rgb_test() {
    let img: Image<f64> = setup("images/tests/colorspace/rgb_hsv.png").unwrap().into();

    let now = SystemTime::now();
    let proc = colorspace::hsv_to_rgb_f32(&convert::scale_channels(&img, 0.0, 0.0, 255.0, 1.0).unwrap());
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/tests/colorspace/hsv_rgb.png").unwrap();
}