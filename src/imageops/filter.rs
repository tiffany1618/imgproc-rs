use crate::image::Image;
use crate::util::Number;
use crate::util::math::apply_1d_kernel;

use core::ops::Mul;

use rulinalg::matrix::{Matrix, BaseMatrix};

pub fn linear_filter(input: &Image<f64>, kernel: &[f64]) -> Option<Image<f64>> {
    let size = (kernel.len() as f32).sqrt() as usize;

    // Check if kernel is a square matrix
    if kernel.len() != size * size {
        return None;
    }

    let kernel_mat = Matrix::new(size, size, kernel);
    let (width, height, channels) = input.dimensions_with_channels();
    let size = kernel_mat.cols();
    let (s, u, v) = kernel_mat.svd().unwrap();
    let mut is_separable = true;

    // Check if kernel is separable
    if s.data()[0] != 0.0 {
        for i in 1..size {
            if s.data()[i * size + i] != 0.0 {
                is_separable = false;
                break;
            }
        }
    } else {
        is_separable = false;
    }

    if is_separable {
        let scalar = s.data()[0].sqrt();
        let vertical_kernel = (u.col(0).as_slice().into_matrix() * scalar).into_vec();
        let horizontal_kernel = (v.transpose().row(0).as_slice().into_matrix() * scalar).into_vec();

        let output_vertical = vertical_filter(input, &vertical_kernel);
        Some(horizontal_filter(&output_vertical, &horizontal_kernel))
    } else {
        // f64ODO: Apply linear filter
        return None;
    }
}

pub fn vertical_filter(input: &Image<f64>, kernel: &[f64]) -> Image<f64> {
    let (width, height, channels) = input.dimensions_with_channels();
    let mut output = Image::blank(width, height, channels, input.has_alpha());

    for y in 0..height {
        for x in 0..width {
            let pixel = apply_1d_kernel(&input.get_neighborhood_vec(x, y, kernel.len() as u32, true), kernel).unwrap();
            output.put_pixel(x, y, pixel);
        }
    }

    output
}

pub fn horizontal_filter(input: &Image<f64>, kernel: &[f64]) -> Image<f64> {
    let (width, height, channels) = input.dimensions_with_channels();
    let mut output = Image::blank(width, height, channels, input.has_alpha());

    for y in 0..height {
        for x in 0..width {
            let pixel = apply_1d_kernel(&input.get_neighborhood_vec(x, y, kernel.len() as u32, false), kernel).unwrap();
            output.put_pixel(x, y, pixel);
        }
    }

    output
}

pub fn box_filter(input: &Image<f64>, size: u32) -> Image<f64> {
    let len = (size * size) as usize;
    let kernel = vec![1.0; len];

    let output_vertical = vertical_filter(input, &kernel);
    horizontal_filter(&output_vertical, &kernel)
}

pub fn box_filter_normalized(input: &Image<f64>, size: u32) -> Image<f64> {
    let len = (size * size) as f64;
    let kernel = vec![1.0 / len; len as usize];

    let output_vertical = vertical_filter(input, &kernel);
    horizontal_filter(&output_vertical, &kernel)
}

// pub fn gaussian_blur(input: &Image<f64>, size: u32) -> Image<f64> {
//
// }
//
// pub fn sharpen(input: &Image<f64>) -> Image<f64> {
//
// }
//
// pub fn unsharp_masking (input: &Image<f64>) -> Image<f64> {
//
// }
