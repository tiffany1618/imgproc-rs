//! A module for image filtering operations

pub use self::median::*;
pub use self::bilateral::*;

mod median;
mod bilateral;

use crate::{util, colorspace, error, math};
use crate::image::{Image, BaseImage, Number};
use crate::util::constants::{K_SOBEL_1D_VERT, K_SOBEL_1D_HORZ, K_UNSHARP_MASKING, K_SHARPEN, K_PREWITT_1D_VERT, K_PREWITT_1D_HORZ};
use crate::enums::Thresh;
use crate::error::ImgProcResult;

use rayon::prelude::*;

/////////////////////
// Linear filtering
/////////////////////

/// Applies a 1D filter. If `is_vert` is true, applies `kernel`
/// as a vertical filter; otherwise applies `kernel` as a horizontal filter
pub fn filter_1d(input: &Image<f64>, kernel: &[f64], is_vert: bool) -> ImgProcResult<Image<f64>> {
    error::check_odd(kernel.len(), "kernel length")?;

    let (width, height) = input.info().wh();
    let mut output = Image::blank(input.info());

    for y in 0..height {
        for x in 0..width {
            let pixel = math::apply_1d_kernel(input.get_neighborhood_1d(x, y,
                                                        kernel.len() as u32, is_vert), kernel)?;
            output.set_pixel(x, y, &pixel);
        }
    }

    Ok(output)
}

/// (Parallel) Applies a 1D filter. If `is_vert` is true, applies `kernel`
/// as a vertical filter; otherwise applies `kernel` as a horizontal filter
pub fn filter_1d_par(input: &Image<f64>, kernel: &[f64], is_vert: bool) -> ImgProcResult<Image<f64>> {
    error::check_odd(kernel.len(), "kernel length")?;

    let (width, height, channels, alpha) = input.info().whca();

    let data: Vec<Vec<f64>> = (0..input.info().size())
        .into_par_iter()
        .map(|i| {
            let (x, y) = util::get_2d_coords(i, width);
            math::apply_1d_kernel(input.get_neighborhood_1d(x, y,kernel.len() as u32, is_vert), kernel).unwrap()
        })
        .collect();

    Ok(Image::from_vec_of_vec(width, height, channels, alpha, data))
}

/// Applies a separable linear filter by first applying `vert_kernel` and then `horz_kernel`
pub fn separable_filter(input: &Image<f64>, vert_kernel: &[f64], horz_kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    error::check_odd(vert_kernel.len(), "vert_kernel length")?;
    error::check_odd(horz_kernel.len(), "horz_kernel length")?;
    error::check_equal(vert_kernel.len(), horz_kernel.len(), "kernel lengths")?;

    let vertical = filter_1d(input, vert_kernel, true)?;
    Ok(filter_1d(&vertical, horz_kernel, false)?)
}

/// (Parallel) Applies a separable linear filter by first applying `vert_kernel` and then `horz_kernel`
pub fn separable_filter_par(input: &Image<f64>, vert_kernel: &[f64], horz_kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    error::check_odd(vert_kernel.len(), "vert_kernel length")?;
    error::check_odd(horz_kernel.len(), "horz_kernel length")?;
    error::check_equal(vert_kernel.len(), horz_kernel.len(), "kernel lengths")?;

    let vertical = filter_1d_par(input, vert_kernel, true)?;
    Ok(filter_1d_par(&vertical, horz_kernel, false)?)
}

/// Applies an unseparable linear filter
pub fn unseparable_filter(input: &Image<f64>, kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    error::check_odd(kernel.len(), "kernel length")?;
    error::check_square(kernel.len() as f64, "kernel length")?;

    let size = (kernel.len() as f32).sqrt() as u32;
    let (width, height) = input.info().wh();
    let mut output = Image::blank(input.info());

    for y in 0..height {
        for x in 0..width {
            let pixel = math::apply_2d_kernel(input.get_neighborhood_2d(x, y, size), kernel)?;
            output.set_pixel(x, y, &pixel);
        }
    }

    Ok(output)
}

/// (Parallel) Applies an unseparable linear filter
pub fn unseparable_filter_par(input: &Image<f64>, kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    error::check_odd(kernel.len(), "kernel length")?;
    error::check_square(kernel.len() as f64, "kernel length")?;

    let size = (kernel.len() as f32).sqrt() as u32;
    let (width, height, channels, alpha) = input.info().whca();

    let data: Vec<Vec<f64>> = (0..input.info().size())
        .into_par_iter()
        .map(|i| {
            let (x, y) = util::get_2d_coords(i, width);
            math::apply_2d_kernel(input.get_neighborhood_2d(x, y, size), kernel).unwrap()
        })
        .collect();

    Ok(Image::from_vec_of_vec(width, height, channels, alpha, data))
}

/// Applies a linear filter using the 2D `kernel`
pub fn linear_filter(input: &Image<f64>, kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    error::check_odd(kernel.len(), "kernel length")?;
    error::check_square(kernel.len() as f64, "kernel length")?;

    let separable = math::separate_kernel(kernel);
    match separable {
        Some((vert, horz)) => Ok(separable_filter(input, &vert, &horz)?),
        None => Ok(unseparable_filter(input, &kernel)?)
    }
}

/// (Parallel) Applies a linear filter using the 2D `kernel`
pub fn linear_filter_par(input: &Image<f64>, kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    error::check_odd(kernel.len(), "kernel length")?;
    error::check_square(kernel.len() as f64, "kernel length")?;

    let separable = math::separate_kernel(kernel);
    match separable {
        Some((vert, horz)) => Ok(separable_filter_par(input, &vert, &horz)?),
        None => Ok(unseparable_filter_par(input, &kernel)?)
    }
}

//////////////
// Blurring
//////////////

/// Applies a box filter using a `size x size` kernel
pub fn box_filter(input: &Image<f64>, size: u32) -> ImgProcResult<Image<f64>> {
    error::check_odd(size, "size")?;

    let len = (size * size) as usize;
    let kernel = vec![1.0; len];

    Ok(separable_filter(input, &kernel, &kernel)?)
}

/// (Parallel) Applies a box filter using a `size x size` kernel
pub fn box_filter_par(input: &Image<f64>, size: u32) -> ImgProcResult<Image<f64>> {
    error::check_odd(size, "size")?;

    let len = (size * size) as usize;
    let kernel = vec![1.0; len];

    Ok(separable_filter_par(input, &kernel, &kernel)?)
}

/// Applies a normalized box filter using a `size x size` kernel
pub fn box_filter_normalized(input: &Image<f64>, size: u32) -> ImgProcResult<Image<f64>> {
    error::check_odd(size, "size")?;

    let len = (size * size) as usize;
    let kernel = vec![1.0 / ((size * size) as f64); len];

    Ok(separable_filter(input, &kernel, &kernel)?)
}

/// (Parallel) Applies a normalized box filter using a `size x size` kernel
pub fn box_filter_normalized_par(input: &Image<f64>, size: u32) -> ImgProcResult<Image<f64>> {
    error::check_odd(size, "size")?;

    let len = (size * size) as usize;
    let kernel = vec![1.0 / ((size * size) as f64); len];

    Ok(separable_filter_par(input, &kernel, &kernel)?)
}

/// Applies a weighted average filter using a `size x size` kernel with a center weight of `weight`
pub fn weighted_avg_filter(input: &Image<f64>, size: u32, weight: u32) -> ImgProcResult<Image<f64>> {
    error::check_odd(size, "size")?;

    let sum = (size * size) - 1 + weight;
    let center = (size / 2) * size + (size / 2);
    let mut kernel = vec![1.0 / (sum as f64); (size * size) as usize];
    kernel[center as usize] = (weight as f64) / (sum as f64);

    Ok(unseparable_filter(input, &kernel)?)
}

/// (Parallel) Applies a weighted average filter using a `size x size` kernel with a center weight of `weight`
pub fn weighted_avg_filter_par(input: &Image<f64>, size: u32, weight: u32) -> ImgProcResult<Image<f64>> {
    error::check_odd(size, "size")?;

    let sum = (size * size) - 1 + weight;
    let center = (size / 2) * size + (size / 2);
    let mut kernel = vec![1.0 / (sum as f64); (size * size) as usize];
    kernel[center as usize] = (weight as f64) / (sum as f64);

    Ok(unseparable_filter_par(input, &kernel)?)
}

/// Applies a Gaussian blur using a `size x size` kernel
pub fn gaussian_blur(input: &Image<f64>, size: u32, std_dev: f64) -> ImgProcResult<Image<f64>> {
    let kernel = util::generate_gaussian_kernel(size, std_dev)?;
    Ok(linear_filter(input, &kernel)?)
}

/// (Parallel) Applies a Gaussian blur using a `size x size` kernel
pub fn gaussian_blur_par(input: &Image<f64>, size: u32, std_dev: f64) -> ImgProcResult<Image<f64>> {
    let kernel = util::generate_gaussian_kernel(size, std_dev)?;
    Ok(linear_filter_par(input, &kernel)?)
}

////////////////
// Sharpening
////////////////

/// Sharpens image
pub fn sharpen(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(unseparable_filter(input, &K_SHARPEN)?)
}

/// (Parallel) Sharpens image
pub fn sharpen_par(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(unseparable_filter_par(input, &K_SHARPEN)?)
}

/// Sharpens image by applying the unsharp masking kernel
pub fn unsharp_masking(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(unseparable_filter(input, &K_UNSHARP_MASKING)?)
}

/// (Parallel) Sharpens image by applying the unsharp masking kernel
pub fn unsharp_masking_par(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(unseparable_filter_par(input, &K_UNSHARP_MASKING)?)
}

////////////////////
// Edge detection
////////////////////

/// Applies a separable derivative mask; first converts `input` to grayscale
pub fn derivative_mask(input: &Image<f64>, vert_kernel: &[f64], horz_kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    let gray = colorspace::rgb_to_grayscale_f64(input);
    let img_x = separable_filter(&gray, &vert_kernel, &horz_kernel)?;
    let img_y = separable_filter(&gray, &horz_kernel, &vert_kernel)?;

    let mut output = Image::blank(gray.info());

    for i in 0..(output.info().full_size() as usize) {
        output.set_pixel_indexed(i, &[(img_x[i][0].powf(2.0) + img_y[i][0].powf(2.0)).sqrt()]);
    }

    Ok(output)
}

/// (Parallel) Applies a separable derivative mask; first converts `input` to grayscale
pub fn derivative_mask_par(input: &Image<f64>, vert_kernel: &[f64], horz_kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    let gray = colorspace::rgb_to_grayscale_f64(input);
    let img_x = separable_filter_par(&gray, &vert_kernel, &horz_kernel)?;
    let img_y = separable_filter_par(&gray, &horz_kernel, &vert_kernel)?;
    let mut output = Image::blank(gray.info());

    for i in 0..(output.info().full_size() as usize) {
        output.set_pixel_indexed(i, &[(img_x[i][0].powf(2.0) + img_y[i][0].powf(2.0)).sqrt()]);
    }

    Ok(output)
}

/// Applies the Prewitt operator
pub fn prewitt(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(derivative_mask(input, &K_PREWITT_1D_VERT, &K_PREWITT_1D_HORZ)?)
}

/// (Parallel) Applies the Prewitt operator
pub fn prewitt_par(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(derivative_mask_par(input, &K_PREWITT_1D_VERT, &K_PREWITT_1D_HORZ)?)
}

/// Applies the Sobel operator
pub fn sobel(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(derivative_mask(input, &K_SOBEL_1D_VERT, &K_SOBEL_1D_HORZ)?)
}

/// (Parallel) Applies the Sobel operator
pub fn sobel_par(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(derivative_mask_par(input, &K_SOBEL_1D_VERT, &K_SOBEL_1D_HORZ)?)
}

/// Applies a Sobel operator with weight `weight`
pub fn sobel_weighted(input: &Image<f64>, weight: u32) -> ImgProcResult<Image<f64>> {
    let vert_kernel = vec![1.0, weight as f64, 1.0];
    Ok(derivative_mask(input, &vert_kernel, &K_SOBEL_1D_HORZ)?)
}

/// (Parallel) Applies a Sobel operator with weight `weight`
pub fn sobel_weighted_par(input: &Image<f64>, weight: u32) -> ImgProcResult<Image<f64>> {
    let vert_kernel = vec![1.0, weight as f64, 1.0];
    Ok(derivative_mask_par(input, &vert_kernel, &K_SOBEL_1D_HORZ)?)
}

//////////////////
// Thresholding
//////////////////

/// Performs a thresholding operation based on `method`
pub fn threshold(input: &Image<f64>, threshold: f64, max: f64, method: Thresh) -> ImgProcResult<Image<f64>> {
    error::check_grayscale(input.info().channels, input.info().alpha)?;

    match method {
        Thresh::Binary => {
            Ok(input.map_channels_if_alpha(|channel| {
                if channel > threshold {
                    max
                } else {
                    0.0
                }
            }, |a| a))
        },
        Thresh::BinaryInv => {
            Ok(input.map_channels_if_alpha(|channel| {
                if channel > threshold {
                    0.0
                } else {
                    max
                }
            }, |a| a))
        },
        Thresh::Trunc => {
            Ok(input.map_channels_if_alpha(|channel| {
                if channel > threshold {
                    threshold
                } else {
                    channel
                }
            }, |a| a))
        },
        Thresh::ToZero => {
            Ok(input.map_channels_if_alpha(|channel| {
                if channel > threshold {
                    channel
                } else {
                    0.0
                }
            }, |a| a))
        },
        Thresh::ToZeroInv => {
            Ok(input.map_channels_if_alpha(|channel| {
                if channel > threshold {
                    0.0
                } else {
                    channel
                }
            }, |a| a))
        },
    }
}

//////////
// Other
//////////

/// Returns the residual image of a filter operation
pub fn residual<T: Number>(original: &Image<T>, filtered: &Image<T>) -> ImgProcResult<Image<T>> {
    error::check_equal(original.info(), filtered.info(), "image dimensions")?;

    let (width, height, channels, alpha) = original.info().whca();
    let mut data = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let p_1 = original.get_pixel(x, y);
            let p_2 = filtered.get_pixel(x, y);

            for c in 0..(channels as usize) {
                data.push(p_1[c] - p_2[c]);
            }
        }
    }

    Ok(Image::from_slice(width, height, channels, alpha, &data))
}