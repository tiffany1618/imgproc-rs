use crate::image::{Image, Pixel};
use crate::imageops::colorspace;
use crate::util::math::{apply_1d_kernel, apply_2d_kernel};
use crate::util::constant::{K_GAUSSIAN_BLUR_1D_3, K_GAUSSIAN_BLUR_1D_5, K_SOBEL_1D_VERT, K_SOBEL_1D_HORZ, K_UNSHARP_MASKING, K_SHARPEN, K_PREWITT_1D_VERT, K_PREWITT_1D_HORZ};
use crate::error::{ImgProcError, ImgProcResult};

use rulinalg::matrix::{Matrix, BaseMatrix};

use std::f64::consts::{PI, E};

/////////////////////
// Linear filtering
/////////////////////

/// Applies a 1D filter. If `is_vert` is true, applies `kernel`
/// as a vertical filter; otherwise applies `kernel` as a horizontal filter
pub fn filter_1d(input: &Image<f64>, kernel: &[f64], is_vert: bool) -> ImgProcResult<Image<f64>> {
    if kernel.len() % 2 == 0 {
        return Err(ImgProcError::InvalidArgument("kernel length is not odd".to_string()));
    }

    let (width, height, channels) = input.dimensions_with_channels();
    let mut output = Image::blank(width, height, channels, input.has_alpha());

    for y in 0..height {
        for x in 0..width {
            let pixel = apply_1d_kernel(&input.get_neighborhood_vec(x, y, kernel.len() as u32, is_vert), kernel)?;
            output.put_pixel(x, y, pixel);
        }
    }

    Ok(output)
}

/// Applies a separable linear filter by first applying `vert_kernel` and then `horz_kernel`
pub fn separable_filter(input: &Image<f64>, vert_kernel: &[f64], horz_kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    if vert_kernel.len() % 2 == 0 || horz_kernel.len() % 2 == 0 {
        return Err(ImgProcError::InvalidArgument("kernel lengths are not odd".to_string()));
    } else if vert_kernel.len() != horz_kernel.len() {
        return Err(ImgProcError::InvalidArgument("kernel lengths are not equal".to_string()));
    }

    let vertical = filter_1d(input, vert_kernel, true)?;
    Ok(filter_1d(&vertical, horz_kernel, false)?)
}

/// Applies an unseparable linear filter
pub fn unseparable_filter(input: &Image<f64>, kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    let size = (kernel.len() as f32).sqrt() as u32;
    if kernel.len() != (size * size) as usize {
        return Err(ImgProcError::InvalidArgument("kernel length is not odd".to_string()));
    }

    let (width, height, channels) = input.dimensions_with_channels();
    let mut output = Image::blank(width, height, channels, input.has_alpha());

    for y in 0..height {
        for x in 0..width {
            let pixel = apply_2d_kernel(&input.get_neighborhood_square(x, y, size), kernel)?;
            output.put_pixel(x, y, pixel);
        }
    }

    Ok(output)
}

/// Applies a linear filter using the 2D `kernel`
pub fn linear_filter(input: &Image<f64>, kernel: &[f64]) -> ImgProcResult<Image<f64>> {
    let size = (kernel.len() as f32).sqrt() as usize;

    // Check if kernel is a square matrix
    if kernel.len() != size * size {
        return Err(ImgProcError::InvalidArgument("kernel is not a square matrix".to_string()));
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

//////////
// Blur
//////////

/// Applies a box filter of odd size `size`
pub fn box_filter(input: &Image<f64>, size: u32) -> ImgProcResult<Image<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgument("size is not odd".to_string()));
    }

    let len = (size * size) as usize;
    let kernel = vec![1.0; len];

    Ok(separable_filter(input, &kernel, &kernel)?)
}

/// Applies a normalized box filter of odd size `size`
pub fn box_filter_normalized(input: &Image<f64>, size: u32) -> ImgProcResult<Image<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgument("size is not odd".to_string()));
    }

    let len = (size * size) as usize;
    let kernel = vec![1.0 / ((size * size) as f64); len];

    Ok(separable_filter(input, &kernel, &kernel)?)
}

/// Applies a weighted average filter of odd size `size` with a center weight of `weight`
pub fn weighted_avg_filter(input: &Image<f64>, size: u32, weight: u32) -> ImgProcResult<Image<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgument("size is not odd".to_string()));
    }

    let sum = (size * size) - 1 + weight;
    let center = (size / 2) * size + (size / 2);
    let mut kernel = vec![1.0 / (sum as f64); (size * size) as usize];
    kernel[center as usize] = (weight as f64) / (sum as f64);

    Ok(unseparable_filter(input, &kernel)?)
}

/// Applies a Gaussian blur of odd size `size`. Currently only supports sizes of 3 and 5
pub fn gaussian_blur(input: &Image<f64>, size: u32, std_dev: f64) -> ImgProcResult<Image<f64>> {
    let kernel = generate_gaussian_kernel(size, std_dev)?;
    Ok(linear_filter(input, &kernel)?)
}

pub fn generate_gaussian_kernel(size: u32, std_dev: f64) -> ImgProcResult<Vec<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgument("size is not odd".to_string()));
    }

    let mut filter = vec![0.0; (size * size) as usize];
    let k = (size - 1) / 2;

    for i in 0..size {
        for j in 0..size {
            if i <= j {
                let num = (1.0 / (2.0 * PI * std_dev * std_dev)) *
                    (E.powf(-(((i - k) * (i - k) + (j - k) * (j - k)) as f64) / (2.0 * std_dev * std_dev)));
                filter[(i * size + j) as usize] = num;

                if i != j {
                    filter[(j * size + i) as usize] = num;
                }
            }
        }
    }

    Ok(filter)
}

/////////////
// Sharpen
/////////////

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

    let (width, height, channels) = gray.dimensions_with_channels();
    let mut output = Image::blank(width, height, channels, input.has_alpha());

    for y in 0..height {
        for x in 0..width {
            let channel = (img_x.get_pixel(x, y).channels()[0].powf(2.0)
                + img_y.get_pixel(x, y).channels()[0].powf(2.0)).sqrt();
            output.put_pixel(x, y, Pixel::new(&vec![channel]));
        }
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
