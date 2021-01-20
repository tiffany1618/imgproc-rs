use crate::image::{Image, Pixel};
use crate::imageops::colorspace;
use crate::util::math::{apply_1d_kernel, apply_2d_kernel};
use crate::util::constant::{K_GAUSSIAN_BLUR_1D_3, K_GAUSSIAN_BLUR_1D_5, K_SOBEL_1D_VERT, K_SOBEL_1D_HORZ, K_UNSHARP_MASKING, K_SHARPEN};

use rulinalg::matrix::{Matrix, BaseMatrix};

/////////////////////
// Linear filtering
/////////////////////

/// Applies a 1D filter. If `is_vert` is true, applies `kernel`
/// as a vertical filter; otherwise applies `kernel` as a horizontal filter
pub fn filter_1d(input: &Image<f64>, kernel: &[f64], is_vert: bool) -> Option<Image<f64>> {
    if kernel.len() % 2 == 0 {
        return None;
    }

    let (width, height, channels) = input.dimensions_with_channels();
    let mut output = Image::blank(width, height, channels, input.has_alpha());

    for y in 0..height {
        for x in 0..width {
            let pixel = apply_1d_kernel(&input.get_neighborhood_vec(x, y, kernel.len() as u32, is_vert), kernel)?;
            output.put_pixel(x, y, pixel);
        }
    }

    Some(output)
}

/// Applies a separable linear filter by first applying `vert_kernel` and then `horz_kernel`
pub fn separable_filter(input: &Image<f64>, vert_kernel: &[f64], horz_kernel: &[f64]) -> Option<Image<f64>> {
    if vert_kernel.len() % 2 == 0
        || horz_kernel.len() % 2 == 0
        || vert_kernel.len() != horz_kernel.len() {
        return None;
    }

    let vertical = filter_1d(input, vert_kernel, true)?;
    Some(filter_1d(&vertical, horz_kernel, false)?)
}

/// Applies an unseparable linear filter
pub fn unseparable_filter(input: &Image<f64>, kernel: &[f64]) -> Option<Image<f64>> {
    let size = (kernel.len() as f32).sqrt() as u32;
    if kernel.len() != (size * size) as usize {
        return None;
    }

    let (width, height, channels) = input.dimensions_with_channels();
    let mut output = Image::blank(width, height, channels, input.has_alpha());

    for y in 0..height {
        for x in 0..width {
            let pixel = apply_2d_kernel(&input.get_neighborhood_square(x, y, size), kernel)?;
            output.put_pixel(x, y, pixel);
        }
    }

    Some(output)
}

/// Applies a linear filter using the 2D `kernel`
pub fn linear_filter(input: &Image<f64>, kernel: &[f64]) -> Option<Image<f64>> {
    println!("a");
    let size = (kernel.len() as f32).sqrt() as usize;

    // Check if kernel is a square matrix
    if kernel.len() != size * size {
        return None;
    }

    let kernel_mat = Matrix::new(size, size, kernel);
    let size = kernel_mat.cols();
    let (s, u, v) = kernel_mat.svd().ok()?;
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
        println!("separable");
        let scalar = s.data()[0].sqrt();
        let vertical_kernel = (u.col(0).as_slice().into_matrix() * scalar).into_vec();
        let horizontal_kernel = (v.transpose().row(0).as_slice().into_matrix() * scalar).into_vec();

        Some(separable_filter(input, &vertical_kernel, &horizontal_kernel)?)
    } else {
        Some(unseparable_filter(input, &kernel)?)
    }
}

//////////
// Blur
//////////

/// Applies a box filter of odd size `size`
pub fn box_filter(input: &Image<f64>, size: u32) -> Option<Image<f64>> {
    if size % 2 == 0 {
        return None;
    }

    let len = (size * size) as usize;
    let kernel = vec![1.0; len];

    Some(separable_filter(input, &kernel, &kernel)?)
}

/// Applies a normalized box filter of odd size `size`
pub fn box_filter_normalized(input: &Image<f64>, size: u32) -> Option<Image<f64>> {
    if size % 2 == 0 {
        return None;
    }

    let len = (size * size) as usize;
    let kernel = vec![1.0 / ((size * size) as f64); len];

    Some(separable_filter(input, &kernel, &kernel)?)
}

/// Applies a Gaussian blur of odd size `size`. Currently only supports sizes of 3 and 5
pub fn gaussian_blur(input: &Image<f64>, size: u32) -> Option<Image<f64>> {
    match size {
        3 => {
            Some(separable_filter(input, &K_GAUSSIAN_BLUR_1D_3, &K_GAUSSIAN_BLUR_1D_3)?)
        },
        5 => {
            Some(separable_filter(input, &K_GAUSSIAN_BLUR_1D_5, &K_GAUSSIAN_BLUR_1D_5)?)
        },
        _ => None
    }
}

/////////////
// Sharpen
/////////////

/// Sharpens image
pub fn sharpen(input: &Image<f64>) -> Option<Image<f64>> {
    Some(unseparable_filter(input, &K_SHARPEN)?)
}

/// Sharpens image by applying the unsharp masking kernel
pub fn unsharp_masking(input: &Image<f64>) -> Option<Image<f64>> {
    Some(unseparable_filter(input, &K_UNSHARP_MASKING)?)
}

////////////////////
// Edge detection
////////////////////

/// Applies the Sobel operator
pub fn sobel(input: &Image<f64>) -> Option<Image<f64>> {
    let gray = colorspace::rgb_to_grayscale_f64(input);
    let img_x = separable_filter(&gray, &K_SOBEL_1D_VERT, &K_SOBEL_1D_HORZ)?;
    let img_y = separable_filter(&gray, &K_SOBEL_1D_HORZ, &K_SOBEL_1D_VERT)?;

    let (width, height, channels) = gray.dimensions_with_channels();
    let mut output = Image::blank(width, height, channels, input.has_alpha());

    for y in 0..height {
        for x in 0..width {
            let channel = (img_x.get_pixel(x, y).channels()[0].powf(2.0)
                + img_y.get_pixel(x, y).channels()[0].powf(2.0)).sqrt();
            output.put_pixel(x, y, Pixel::new(&vec![channel]));
        }
    }

    Some(output)
}

