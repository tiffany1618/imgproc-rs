#![allow(dead_code)]

mod common;

use common::setup;
use imgproc_rs::filter;
use imgproc_rs::image::Image;
use imgproc_rs::io::write;

use std::time::SystemTime;
use imgproc_rs::enums::Bilateral;

// #[test]
fn box_filter_par() {
    let img: Image<f64> = setup().unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::box_filter_par(&img, 5).unwrap();
    println!("box filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/box_filter.png").unwrap();
}

// #[test]
fn box_filter_normalized_par() {
    let img: Image<f64> = setup().unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::box_filter_normalized_par(&img, 5).unwrap();
    println!("box filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/box_filter_norm.png").unwrap();
}

// #[test]
fn weighted_avg_filter_par() {
    let img: Image<f64> = setup().unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::weighted_avg_filter_par(&img, 5, 5).unwrap();
    println!("weighted avg filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/weighted_avg.png").unwrap();
}

// #[test]
fn gaussian_blur_par() {
    let img: Image<f64> = setup().unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::gaussian_blur_par(&img, 5, 1.0).unwrap();
    println!("gaussian blur filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/gaussian_blur.png").unwrap();
}

// #[test]
fn median_filter() {
    let img = setup().unwrap();

    let now = SystemTime::now();
    let filtered = filter::median_filter(&img, 5).unwrap();
    println!("median filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/median.png").unwrap();
}

// #[test]
fn alpha_trimmed_mean_filter() {
    let img = setup().unwrap();

    let now = SystemTime::now();
    let filtered = filter::alpha_trimmed_mean_filter(&img, 5, 2).unwrap();
    println!("alpha trimmed mean filter: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/alpha.png").unwrap();
}

// #[test]
fn bilateral_filter_par() {
    let img = setup().unwrap();

    let now = SystemTime::now();
    let direct = filter::bilateral_filter_par(&img, 10.0, 4.0, Bilateral::Direct).unwrap();
    println!("bilateral direct: {}", now.elapsed().unwrap().as_millis());

    write(&direct.into(), "images/tests/filter/bilateral_direct.png").unwrap();
}

// #[test]
fn sharpen_par() {
    let img: Image<f64> = setup().unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::sharpen_par(&img).unwrap();
    println!("sharpen: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/sharpen.png").unwrap();
}

// #[test]
fn unsharp_masking_par() {
    let img: Image<f64> = setup().unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::unsharp_masking_par(&img).unwrap();
    println!("unsharp masking: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/unsharp_masking.png").unwrap();
}

// #[test]
fn prewitt_par() {
    let img: Image<f64> = setup().unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::prewitt_par(&img).unwrap();
    println!("prewitt: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/prewitt.png").unwrap();
}

// #[test]
fn sobel_par() {
    let img: Image<f64> = setup().unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::sobel_par(&img).unwrap();
    println!("sobel: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/sobel.png").unwrap();
}

// #[test]
fn sobel_weighted_par() {
    let img: Image<f64> = setup().unwrap().into();

    let now = SystemTime::now();
    let filtered = filter::sobel_weighted_par(&img, 5).unwrap();
    println!("sobel weighted: {}", now.elapsed().unwrap().as_millis());

    write(&filtered.into(), "images/tests/filter/sobel_weighted.png").unwrap();
}
