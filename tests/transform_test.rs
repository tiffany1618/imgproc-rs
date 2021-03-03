#![allow(dead_code)]

mod common;

use common::setup;
use imgproc_rs::transform;
use imgproc_rs::image::{BaseImage, Image};
use imgproc_rs::io::write;

use std::time::SystemTime;
use imgproc_rs::enums::{Scale, Refl};

const PATH: &str = "images/beach.jpg";

// #[test]
fn crop() {
    let img = setup(PATH).unwrap();
    let (width, height) = img.info().wh();

    let now = SystemTime::now();
    let output = transform::crop(&img, 0, 0, width/2, height/2).unwrap();
    println!("crop: {}", now.elapsed().unwrap().as_millis());

    write(&output, "images/tests/transform/crop.png").unwrap();
}

// #[test]
fn superimpose() {
    let img: Image<f64> = setup(PATH).unwrap().into();
    let (width, height) = img.info().wh();

    let now = SystemTime::now();
    let output = transform::superimpose(&img, &img, width/2, height/2, 0.5).unwrap();
    println!("superimpose: {}", now.elapsed().unwrap().as_millis());

    write(&output.into(), "images/tests/transform/superimpose.png").unwrap();
}

// #[test]
fn overlay() {
    let img = setup(PATH).unwrap();
    let (width, height) = img.info().wh();

    let now = SystemTime::now();
    let output = transform::overlay(&img, &img, width/2, height/2).unwrap();
    println!("overlay: {}", now.elapsed().unwrap().as_millis());

    write(&output.into(), "images/tests/transform/overlay.png").unwrap();
}

#[test]
fn scale_twice() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let mut now = SystemTime::now();
    let nearest = transform::scale(&img, 2.0, 2.0, Scale::NearestNeighbor).unwrap();
    println!("nearest: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let bilinear = transform::scale(&img, 2.0, 2.0, Scale::Bilinear).unwrap();
    println!("bilinear: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let bicubic = transform::scale(&img, 2.0, 2.0, Scale::Bicubic).unwrap();
    println!("bicubic: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let lanczos = transform::scale(&img, 2.0, 2.0, Scale::Lanczos).unwrap();
    println!("lanczos: {}", now.elapsed().unwrap().as_millis());

    write(&nearest.into(), "images/tests/transform/scale_nearest_twice.png").unwrap();
    write(&bilinear.into(), "images/tests/transform/scale_bilinear_twice.png").unwrap();
    write(&bicubic.into(), "images/tests/transform/scale_bicubic_twice.png").unwrap();
    write(&lanczos.into(), "images/tests/transform/scale_lanczos_twice.png").unwrap();
}

// #[test]
fn scale_half() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let mut now = SystemTime::now();
    let nearest = transform::scale(&img, 0.5, 0.5, Scale::NearestNeighbor).unwrap();
    println!("nearest: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let bilinear = transform::scale(&img, 0.5, 0.5, Scale::Bilinear).unwrap();
    println!("bilinear: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let bicubic = transform::scale(&img, 0.5, 0.5, Scale::Bicubic).unwrap();
    println!("bicubic: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let lanczos = transform::scale(&img, 0.5, 0.5, Scale::Lanczos).unwrap();
    println!("lanczos: {}", now.elapsed().unwrap().as_millis());

    write(&nearest.into(), "images/tests/transform/scale_nearest_half.png").unwrap();
    write(&bilinear.into(), "images/tests/transform/scale_bilinear_half.png").unwrap();
    write(&bicubic.into(), "images/tests/transform/scale_bicubic_half.png").unwrap();
    write(&lanczos.into(), "images/tests/transform/scale_lanczos_half.png").unwrap();
}

// #[test]
fn translate() {
    let img = setup(PATH).unwrap();
    let (width, height) = img.info().wh();

    let now = SystemTime::now();
    let output = transform::translate(&img, width/2, height/2).unwrap();
    println!("translate: {}", now.elapsed().unwrap().as_millis());

    write(&output, "images/tests/transform/translate.png").unwrap();
}

// #[test]
fn rotate() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let mut now = SystemTime::now();
    let output_90 = transform::rotate(&img, 90.0).unwrap();
    println!("rotate 90: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let output_180 = transform::rotate(&img, 180.0).unwrap();
    println!("rotate 180: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let output_270 = transform::rotate(&img, 270.0).unwrap();
    println!("rotate 270: {}", now.elapsed().unwrap().as_millis());

    write(&output_90.into(), "images/tests/transform/rotate_90.png").unwrap();
    write(&output_180.into(), "images/tests/transform/rotate_180.png").unwrap();
    write(&output_270.into(), "images/tests/transform/rotate_270.png").unwrap();
}

// #[test]
fn reflect() {
    let img = setup(PATH).unwrap();

    let mut now = SystemTime::now();
    let output_horz = transform::reflect(&img, Refl::Horizontal).unwrap();
    println!("horizontal: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let output_vert = transform::reflect(&img, Refl::Vertical).unwrap();
    println!("vertical: {}", now.elapsed().unwrap().as_millis());

    write(&output_horz, "images/tests/transform/reflect_horz.png").unwrap();
    write(&output_vert, "images/tests/transform/reflect_vert.png").unwrap();
}

// #[test]
fn shear() {
    let img: Image<f64> = setup(PATH).unwrap().into();

    let mut now = SystemTime::now();
    let output_pp = transform::shear(&img, 0.5, 0.0).unwrap();
    println!("shear px: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let output_np = transform::shear(&img, -0.5, 0.0).unwrap();
    println!("shear nx: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let output_pn = transform::shear(&img, 0.0, 0.5).unwrap();
    println!("shear py: {}", now.elapsed().unwrap().as_millis());

    now = SystemTime::now();
    let output_nn = transform::shear(&img, 0.0, -0.5).unwrap();
    println!("shear ny: {}", now.elapsed().unwrap().as_millis());

    write(&output_pp.into(), "images/tests/transform/shear_px.png").unwrap();
    write(&output_np.into(), "images/tests/transform/shear_nx.png").unwrap();
    write(&output_pn.into(), "images/tests/transform/shear_py.png").unwrap();
    write(&output_nn.into(), "images/tests/transform/shear_ny.png").unwrap();
}
