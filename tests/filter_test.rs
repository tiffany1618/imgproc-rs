#![allow(dead_code)]

mod common;

use common::setup;
use imgproc_rs::{filter, colorspace};
use imgproc_rs::image::Image;
use imgproc_rs::io::write;

use std::time::SystemTime;
use imgproc_rs::enums::{Bilateral, Thresh};

const PATH: &str = "images/yosemite.jpg";

// #[test]
fn box_filter() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::box_filter(&img, 5).unwrap();
    println!("box filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/box_filter.png").unwrap();
}

// #[test]
fn weighted_avg_filter() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::weighted_avg_filter(&img, 5, 5).unwrap();
    println!("weighted avg filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/weighted_avg.png").unwrap();
}

// #[test]
fn gaussian_blur() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::gaussian_blur(&img, 3, 1.0).unwrap();
    println!("gaussian blur filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/gaussian_blur.png").unwrap();
}

// #[test]
fn median_filter() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let filtered = filter::median_filter(&img, 5).unwrap();
    println!("median filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/median.png").unwrap();
}

// #[test]
fn alpha_trimmed_mean_filter() {
    let img = setup(PATH).unwrap();

    let now = SystemTime::now();
    let filtered = filter::alpha_trimmed_mean_filter(&img, 5, 2).unwrap();
    println!("alpha trimmed mean filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/alpha.png").unwrap();
}

// #[test]
fn bilateral_filter() {
    let img = setup("images/scaled.png").unwrap();

    let now = SystemTime::now();
    let direct = filter::bilateral_filter(&img, 10.0, 4.0, Bilateral::Direct).unwrap();
    println!("bilateral direct: {}", now.elapsed().unwrap().as_millis());

    write(&direct.into(), "images/tests/filter/bilateral_direct.png").unwrap();
}

// #[test]
fn sharpen() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::sharpen(&img).unwrap();
    println!("sharpen: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/sharpen.png").unwrap();
}

// #[test]
fn unsharp_masking() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::unsharp_masking(&img).unwrap();
    println!("unsharp masking: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/unsharp_masking.png").unwrap();
}

// #[test]
fn prewitt() {
    let img: Image<f64> = setup("images/poppy.jpg").unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::prewitt(&img).unwrap();
    println!("prewitt: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/prewitt.png").unwrap();
}

// #[test]
fn sobel() {
    let img: Image<f64> = setup("images/poppy.jpg").unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::sobel(&img).unwrap();
    println!("sobel: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/sobel.png").unwrap();
}

// #[test]
fn sobel_weighted() {
    let img: Image<f64> = setup("images/poppy.jpg").unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::sobel_weighted(&img, 5).unwrap();
    println!("sobel weighted: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/sobel_weighted.png").unwrap();
}

// #[test]
fn laplacian() {
    let img: Image<f64> = colorspace::rgb_to_grayscale(&setup("images/poppy.jpg").unwrap()).into();

    let now = SystemTime::now();
    let filtered = filter::laplacian(&img).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&filter::normalize_laplacian(&filtered).unwrap(), "images/tests/filter/laplacian.png").unwrap();
}

#[test]
fn laplacian_of_gaussian() {
    let img: Image<f64> = colorspace::rgb_to_grayscale(&setup("images/scaled.png").unwrap()).into();

    let now = SystemTime::now();
    let filtered = filter::laplacian_of_gaussian(&img, 7, 1.0).unwrap();
    println!("processing: {}", now.elapsed().unwrap().as_millis());

    write(&filter::normalize_laplacian(&filtered).unwrap(), "images/tests/filter/laplacian_of_gaussian.png").unwrap();
}

// #[test]
fn threshold_test() {
    let img: Image<f64> = colorspace::rgb_to_grayscale(&setup(PATH).unwrap()).into();

    let mut now = SystemTime::now();
    let bin = filter::threshold(&img, 100.0, 255.0, Thresh::Binary).unwrap();
    println!("bin: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let bin_inv = filter::threshold(&img, 100.0, 255.0, Thresh::BinaryInv).unwrap();
    println!("bin inv: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let trunc = filter::threshold(&img, 100.0, 255.0, Thresh::Trunc).unwrap();
    println!("trunc: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let zero = filter::threshold(&img, 100.0, 255.0, Thresh::ToZero).unwrap();
    println!("zero: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let zero_inv = filter::threshold(&img, 100.0, 255.0, Thresh::ToZeroInv).unwrap();
    println!("zero inv: {}", now.elapsed().unwrap().as_millis());

    write(&bin.into(), "images/tests/filter/thresh_binary.png").unwrap();
    write(&bin_inv.into(), "images/tests/filter/thresh_binary_inv.png").unwrap();
    write(&trunc.into(), "images/tests/filter/thresh_trunc.png").unwrap();
    write(&zero.into(), "images/tests/filter/thresh_to_zero.png").unwrap();
    write(&zero_inv.into(), "images/tests/filter/thresh_to_zero_inv.png").unwrap();
}