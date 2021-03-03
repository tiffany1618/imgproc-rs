#![allow(dead_code)]

mod common;

use common::setup;
use imgproc_rs::colorspace;
use imgproc_rs::image::{BaseImage, Image};
use imgproc_rs::io::write;

use std::time::SystemTime;
use imgproc_rs::enums::White;

const PATH: &str = "images/spectrum.jpg";

#[test]
fn rgb_to_grayscale_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let gray = colorspace::rgb_to_grayscale(&img);
    println!("crop: {}", now.elapsed().unwrap().as_millis());

    write(&gray, "images/test/colorspace/gray.png").unwrap();
}

#[test]
fn rgb_to_grayscale_f64_test() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let now = SystemTime::now();
    let gray = colorspace::rgb_to_grayscale_f64(&img);
    println!("crop: {}", now.elapsed().unwrap().as_millis());

    write(&gray.into(), "images/test/colorspace/gray_f64.png").unwrap();
}

#[test]
fn srgb_to_xyz_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = colorspace::srgb_to_xyz(&img);
    println!("crop: {}", now.elapsed().unwrap().as_millis());

    write(&proc.into(), "images/test/colorspace/srgb_xyz.png").unwrap();
}

#[test]
fn xyz_to_srgb_test() {
    let img: Image<f64> = setup("images/test/colorspace/srgb_xyz.png").unwrap().into();

    let now = SystemTime::now();
    let proc = colorspace::xyz_to_srgb(&img);
    println!("crop: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/test/colorspace/xyz_srgb.png").unwrap();
}

#[test]
fn srgb_to_lab_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = colorspace::srgb_to_lab(&img, &White::D50);
    println!("crop: {}", now.elapsed().unwrap().as_millis());

    write(&proc.into(), "images/test/colorspace/srgb_lab.png").unwrap();
}

#[test]
fn lab_to_srgb_test() {
    let img: Image<f64> = setup("images/test/colorspace/srgb_lab.png").unwrap().into();

    let now = SystemTime::now();
    let proc = colorspace::lab_to_srgb(&img, &White::D50);
    println!("crop: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/test/colorspace/lab_srgb.png").unwrap();
}

#[test]
fn rgb_to_hsv_test() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let proc = colorspace::rgb_to_hsv(&img);
    println!("crop: {}", now.elapsed().unwrap().as_millis());

    write(&proc.into(), "images/test/colorspace/rgb_hsv.png").unwrap();
}

#[test]
fn hsv_to_rgb_test() {
    let img: Image<f64> = setup("images/test/colorspace/rgb_hsv.png").unwrap().into();

    let now = SystemTime::now();
    let proc = colorspace::hsv_to_rgb(&img);
    println!("crop: {}", now.elapsed().unwrap().as_millis());

    write(&proc, "images/test/colorspace/hsv_rgb.png").unwrap();
}