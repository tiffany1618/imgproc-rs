use crate::{error, util};
use crate::enums::Thresh;
use crate::error::ImgProcResult;
use crate::image::{BaseImage, Image, Number};
use crate::util::constants::{K_SHARPEN, K_UNSHARP_MASKING};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/////////////////////
// Linear filtering
/////////////////////

/// Applies a 1D filter. If `is_vert` is true, applies `kernel`
/// as a vertical filter; otherwise applies `kernel` as a horizontal filter
#[cfg(not(feature = "rayon"))]
pub fn filter_1d(input: &Image<f32>, kernel: &[f32], is_vert: bool) -> ImgProcResult<Image<f32>> {
    error::check_odd(kernel.len(), "kernel length")?;

    let (width, height, channels) = input.info().whc();
    let mut output = Image::blank(input.info());
    let mut p_out = Vec::with_capacity(channels as usize);

    for y in 0..height {
        for x in 0..width {
            util::apply_1d_kernel(&input.get_neighborhood_1d(x, y, kernel.len() as u32, is_vert),
                                              &mut p_out, kernel)?;
            output.set_pixel(x, y, &p_out);
        }
    }

    Ok(output)
}

/// Applies a 1D filter. If `is_vert` is true, applies `kernel`
/// as a vertical filter; otherwise applies `kernel` as a horizontal filter
#[cfg(feature = "rayon")]
pub fn filter_1d(input: &Image<f32>, kernel: &[f32], is_vert: bool) -> ImgProcResult<Image<f32>> {
    error::check_odd(kernel.len(), "kernel length")?;

    let (width, height, channels, alpha) = input.info().whca();

    let data: Vec<Vec<f32>> = (0..input.info().size())
        .into_par_iter()
        .map(|i| {
            let (x, y) = util::get_2d_coords(i, width);
            util::apply_1d_kernel(&input.get_neighborhood_1d(x, y,kernel.len() as u32, is_vert), kernel).unwrap()
        })
        .collect();

    Ok(Image::from_vec_of_vec(width, height, channels, alpha, data))
}

/// Applies a separable linear filter by first applying `vert_kernel` and then `horz_kernel`
pub fn separable_filter(input: &Image<f32>, vert_kernel: &[f32], horz_kernel: &[f32]) -> ImgProcResult<Image<f32>> {
    error::check_odd(vert_kernel.len(), "vert_kernel length")?;
    error::check_odd(horz_kernel.len(), "horz_kernel length")?;
    error::check_equal(vert_kernel.len(), horz_kernel.len(), "kernel lengths")?;

    let vertical = filter_1d(input, vert_kernel, true)?;
    Ok(filter_1d(&vertical, horz_kernel, false)?)
}

/// Applies an unseparable linear filter
#[cfg(not(feature = "rayon"))]
pub fn unseparable_filter(input: &Image<f32>, kernel: &[f32]) -> ImgProcResult<Image<f32>> {
    error::check_odd(kernel.len(), "kernel length")?;
    error::check_square(kernel.len() as f32, "kernel length")?;

    let size = (kernel.len() as f32).sqrt() as u32;
    let (width, height, channels) = input.info().whc();
    let mut output = Image::blank(input.info());
    let mut p_out = Vec::with_capacity(channels as usize);

    for y in 0..height {
        for x in 0..width {
            util::apply_2d_kernel(&input.get_neighborhood_2d(x, y, size), &mut p_out, kernel)?;
            output.set_pixel(x, y, &p_out);
        }
    }

    Ok(output)
}

/// Applies an unseparable linear filter
#[cfg(feature = "rayon")]
pub fn unseparable_filter(input: &Image<f32>, kernel: &[f32]) -> ImgProcResult<Image<f32>> {
    error::check_odd(kernel.len(), "kernel length")?;
    error::check_square(kernel.len() as f32, "kernel length")?;

    let size = (kernel.len() as f32).sqrt() as u32;
    let (width, height, channels, alpha) = input.info().whca();

    let data: Vec<Vec<f32>> = (0..input.info().size())
        .into_par_iter()
        .map(|i| {
            let (x, y) = util::get_2d_coords(i, width);
            util::apply_2d_kernel(&input.get_neighborhood_2d(x, y, size), kernel).unwrap()
        })
        .collect();

    Ok(Image::from_vec_of_vec(width, height, channels, alpha, data))
}

/// Applies a linear filter using the 2D `kernel`
pub fn linear_filter(input: &Image<f32>, kernel: &[f32]) -> ImgProcResult<Image<f32>> {
    error::check_odd(kernel.len(), "kernel length")?;
    error::check_square(kernel.len() as f32, "kernel length")?;

    let separable = util::separate_kernel(kernel);
    match separable {
        Some((vert, horz)) => Ok(separable_filter(input, &vert, &horz)?),
        None => Ok(unseparable_filter(input, &kernel)?)
    }
}

//////////////
// Blurring
//////////////

/// Applies a normalized box filter using a `size x size` kernel
pub fn box_filter(input: &Image<f32>, size: u32) -> ImgProcResult<Image<f32>> {
    error::check_odd(size, "size")?;

    let len = (size * size) as usize;
    let kernel = vec![1.0 / ((size * size) as f32); len];

    Ok(separable_filter(input, &kernel, &kernel)?)
}

/// Applies a weighted average filter using a `size x size` kernel with a center weight of `weight`
pub fn weighted_avg_filter(input: &Image<f32>, size: u32, weight: u32) -> ImgProcResult<Image<f32>> {
    error::check_odd(size, "size")?;

    let sum = (size * size) - 1 + weight;
    let center = (size / 2) * size + (size / 2);
    let mut kernel = vec![1.0 / (sum as f32); (size * size) as usize];
    kernel[center as usize] = (weight as f32) / (sum as f32);

    Ok(unseparable_filter(input, &kernel)?)
}

/// Applies a Gaussian blur using a `size x size` kernel
pub fn gaussian_blur(input: &Image<f32>, size: u32, sigma: f32) -> ImgProcResult<Image<f32>> {
    let kernel = util::generate_gaussian_kernel(size, sigma)?;
    Ok(linear_filter(input, &kernel)?)
}

////////////////
// Sharpening
////////////////

/// Sharpens image
pub fn sharpen(input: &Image<f32>) -> ImgProcResult<Image<f32>> {
    Ok(unseparable_filter(input, &K_SHARPEN)?)
}

/// Sharpens image by applying the unsharp masking kernel
pub fn unsharp_masking(input: &Image<f32>) -> ImgProcResult<Image<f32>> {
    Ok(unseparable_filter(input, &K_UNSHARP_MASKING)?)
}

//////////////////
// Thresholding
//////////////////

/// Performs a thresholding operation based on `method`
pub fn threshold(input: &Image<f32>, threshold: f32, max: f32, method: Thresh) -> ImgProcResult<Image<f32>> {
    error::check_grayscale(input)?;

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