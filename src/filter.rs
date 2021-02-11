//! A module for image filtering operations

use crate::image::{Image, BaseImage};
use crate::{util, colorspace};
use crate::math;
use crate::util::constants::{K_SOBEL_1D_VERT, K_SOBEL_1D_HORZ, K_UNSHARP_MASKING, K_SHARPEN, K_PREWITT_1D_VERT, K_PREWITT_1D_HORZ};
use crate::enums::{Thresh, Bilateral, White};
use crate::error::{ImgProcError, ImgProcResult};

use rulinalg::matrix::{Matrix, BaseMatrix};

/////////////////////
// Linear filtering
/////////////////////

/// Applies a 1D filter. If `is_vert` is true, applies `kernel`
/// as a vertical filter; otherwise applies `kernel` as a horizontal filter
pub fn filter_1d(input: &Image<f64>, kernel: &[f64], is_vert: bool) -> ImgProcResult<Image<f64>> {
    if kernel.len() % 2 == 0 {
        return Err(ImgProcError::InvalidArgError("kernel length is not odd".to_string()));
    }

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

/// Applies a separable linear filter by first applying `vert_kernel` and then `horz_kernel`
pub fn separable_filter(input: &Image<f64>, vert_kernel: &[f64], horz_kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    if vert_kernel.len() % 2 == 0 || horz_kernel.len() % 2 == 0 {
        return Err(ImgProcError::InvalidArgError("kernel lengths are not odd".to_string()));
    } else if vert_kernel.len() != horz_kernel.len() {
        return Err(ImgProcError::InvalidArgError("kernel lengths are not equal".to_string()));
    }

    let vertical = filter_1d(input, vert_kernel, true)?;
    Ok(filter_1d(&vertical, horz_kernel, false)?)
}

/// Applies an unseparable linear filter
pub fn unseparable_filter(input: &Image<f64>, kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    let size = (kernel.len() as f32).sqrt() as u32;
    if kernel.len() != (size * size) as usize {
        return Err(ImgProcError::InvalidArgError("kernel length is not odd".to_string()));
    }

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

/// Applies a linear filter using the 2D `kernel`
pub fn linear_filter(input: &Image<f64>, kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    let size = (kernel.len() as f32).sqrt() as usize;

    // Check if kernel is a square matrix
    if kernel.len() != size * size {
        return Err(ImgProcError::InvalidArgError("kernel is not a square matrix".to_string()));
    }

    let kernel_mat = Matrix::new(size, size, kernel);
    let size = kernel_mat.cols();
    let (s, u, v) = kernel_mat.svd()?;
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

        Ok(separable_filter(input, &vertical_kernel, &horizontal_kernel)?)
    } else {
        Ok(unseparable_filter(input, &kernel)?)
    }
}

//////////////
// Blurring
//////////////

/// Applies a box filter of odd size `size`
pub fn box_filter(input: &Image<f64>, size: u32) -> ImgProcResult<Image<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgError("size is not odd".to_string()));
    }

    let len = (size * size) as usize;
    let kernel = vec![1.0; len];

    Ok(separable_filter(input, &kernel, &kernel)?)
}

/// Applies a normalized box filter of odd size `size`
pub fn box_filter_normalized(input: &Image<f64>, size: u32) -> ImgProcResult<Image<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgError("size is not odd".to_string()));
    }

    let len = (size * size) as usize;
    let kernel = vec![1.0 / ((size * size) as f64); len];

    Ok(separable_filter(input, &kernel, &kernel)?)
}

/// Applies a weighted average filter of odd size `size` with a center weight of `weight`
pub fn weighted_avg_filter(input: &Image<f64>, size: u32, weight: u32) -> ImgProcResult<Image<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgError("size is not odd".to_string()));
    }

    let sum = (size * size) - 1 + weight;
    let center = (size / 2) * size + (size / 2);
    let mut kernel = vec![1.0 / (sum as f64); (size * size) as usize];
    kernel[center as usize] = (weight as f64) / (sum as f64);

    Ok(unseparable_filter(input, &kernel)?)
}

/// Applies a Gaussian blur of odd size `size`. Currently only supports sizes of 3 and 5
pub fn gaussian_blur(input: &Image<f64>, size: u32, std_dev: f64) -> ImgProcResult<Image<f64>> {
    let kernel = util::generate_gaussian_kernel(size, std_dev)?;
    Ok(linear_filter(input, &kernel)?)
}

/// Applies a median filter
pub fn median_filter(input: &Image<f64>, size: u32) -> ImgProcResult<Image<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgError("size is not odd".to_string()));
    }

    let (width, height, channels) = input.info().whc();
    let center = ((size * size) / 2) as usize;
    let mut output = Image::blank(input.info());

    for y in 0..height {
        for x in 0..width {
            let pixels = input.get_neighborhood_2d(x, y, size);
            let mut p_out = Vec::new();

            for c in 0..(channels as usize) {
                let mut p_in = Vec::new();

                for i in 0..((size * size) as usize) {
                    p_in.push(pixels[i][c]);
                }

                p_in.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                p_out.push(p_in[center]);
            }


            output.set_pixel(x, y, &p_out);
        }
    }

    Ok(output)
}

/// Applies an alpha-trimmed mean filter
pub fn alpha_trimmed_mean_filter(input: &Image<f64>, size: u32, alpha: u32) -> ImgProcResult<Image<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgError("size is not odd".to_string()));
    } else if alpha % 2 != 0 {
        return Err(ImgProcError::InvalidArgError("alpha is not even".to_string()));
    } else if alpha >= (size * size) {
        return Err(ImgProcError::InvalidArgError(format!("invalid alpha: size is {}, but alpha is {}", size, alpha)));
    }

    let (width, height, channels) = input.info().whc();
    let length = (size * size) as usize;
    let mut output = Image::blank(input.info());

    for y in 0..height {
        for x in 0..width {
            let pixels = input.get_neighborhood_2d(x, y, size);
            let mut p_out = Vec::new();

            for c in 0..(channels as usize) {
                let mut p_in = Vec::new();
                let mut sum = 0.0;

                for i in 0..length {
                    p_in.push(pixels[i][c]);
                    sum += pixels[i][c];
                }

                p_in.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

                for _ in 0..((alpha / 2) as usize) {
                    sum -= p_in[0];
                    sum -= p_in[p_in.len() - 1];
                    p_in.remove(0);
                    p_in.remove(p_in.len() - 1);
                }

                p_out.push(sum / (p_in.len() as f64));
            }

            output.set_pixel(x, y, &p_out);
        }
    }

    Ok(output)
}

/// Applies a bilateral filter
pub fn bilateral_filter(input: &Image<u8>, range: f64, spatial: f64, algorithm: Bilateral)
    -> ImgProcResult<Image<u8>> {
    if range < 0.0 {
        return Err(ImgProcError::InvalidArgError("range must be non-negative".to_string()));
    } else if spatial < 0.0 {
        return Err(ImgProcError::InvalidArgError("spatial must be non-negative".to_string()));
    }

    let (width, height, channels) = input.info().whc();
    let size = ((spatial * 4.0) + 1.0) as u32;
    let spatial_mat = math::generate_spatial_mat(size, spatial)?;

    let lab = colorspace::srgb_to_lab(&input, &White::D65);
    let mut output = Image::blank(lab.info());

    match algorithm {
        Bilateral::Direct => {
            for y in 0..height {
                for x in 0..width {
                    let p_n = lab.get_neighborhood_2d(x, y, size as u32);
                    let p_in = lab.get_pixel(x, y);
                    let mut p_out = Vec::new();

                    for c in 0..(channels as usize) {
                        let mut total_weight = 0.0;
                        let mut p_curr = 0.0;

                        for i in 0..((size * size) as usize) {
                            let g_r = math::gaussian_fn((p_in[c] - p_n[i][c]).abs(), range)?;
                            let weight = spatial_mat[i] * g_r;

                            p_curr += weight * p_n[i][c];
                            total_weight += weight;
                        }

                        p_out.push(p_curr / total_weight);
                    }

                    output.set_pixel(x, y, &p_out);
                }
            }
        },
        Bilateral::Grid => {

        },
        Bilateral::LocalHistogram => {

        },
    }

    Ok(colorspace::lab_to_srgb(&output, &White::D65))
}

////////////////
// Sharpening
////////////////

/// Sharpens image
pub fn sharpen(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(unseparable_filter(input, &K_SHARPEN)?)
}

/// Sharpens image by applying the unsharp masking kernel
pub fn unsharp_masking(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(unseparable_filter(input, &K_UNSHARP_MASKING)?)
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
        output.set_pixel_indexed(i, &[img_x[i][0].powf(2.0) + img_y[i][0].powf(2.0).sqrt()]);
    }

    Ok(output)
}

/// Applies the Prewitt operator
pub fn prewitt(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(derivative_mask(input, &K_PREWITT_1D_VERT, &K_PREWITT_1D_HORZ)?)
}

/// Applies the Sobel operator
pub fn sobel(input: &Image<f64>) -> ImgProcResult<Image<f64>> {
    Ok(derivative_mask(input, &K_SOBEL_1D_VERT, &K_SOBEL_1D_HORZ)?)
}

/// Applies a Sobel operator with weight `weight`
pub fn sobel_weighted(input: &Image<f64>, weight: u32) -> ImgProcResult<Image<f64>> {
    let vert_kernel = vec![1.0, weight as f64, 1.0];
    Ok(derivative_mask(input, &vert_kernel, &K_SOBEL_1D_HORZ)?)
}

//////////////////
// Thresholding
//////////////////

/// Performs a thresholding operation based on `method`
pub fn threshold(input: &Image<f64>, threshold: f64, max: f64, method: Thresh) -> ImgProcResult<Image<f64>> {
    if !util::is_grayscale(input.info().channels, input.info().alpha) {
        return Err(ImgProcError::InvalidArgError("input is not a grayscale image".to_string()));
    }

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