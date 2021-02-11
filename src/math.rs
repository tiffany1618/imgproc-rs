//! A module for utility math functions

use crate::error;
use crate::image::{Number, SubImage, BaseImage};
use crate::error::ImgProcResult;

use std::f64::consts::{PI, E};

/// Returns the result of the multiplication of a square matrix by a vector
pub fn vector_mul<T: Number>(mat: &[T], vec: &[T]) -> ImgProcResult<Vec<T>> {
    let rows = vec.len();
    let mat_cols = mat.len() / rows;

    error::check_equal(mat_cols, rows, "mat and vec dimensions")?;

    let mut output = vec![0.into(); rows];

    for i in 0..rows {
        for j in 0..rows {
            output[i] += mat[rows * i + j] * vec[j];
        }
    }

    Ok(output)
}

/// Returns the maximum of three f64 values
pub fn max_3(x: f64, y: f64, z: f64) -> f64 {
    if x > y {
        if x > z {
            x
        } else {
            z
        }
    } else {
        if y > z {
            y
        } else {
            z
        }
    }
}

/// Returns the minimum of three f64 values
pub fn min_3(x: f64, y: f64, z: f64) -> f64 {
    if x < y {
        if x < z {
            x
        } else {
            z
        }
    } else {
        if y < z {
            y
        } else {
            z
        }
    }
}

/// Returns the maximum of four f64 values
pub fn max_4(w: f64, x: f64, y: f64, z: f64) -> f64 {
    if w > x {
        max_3(w, y, z)
    } else if x > y {
        max_3(w, x, z)
    } else if y > z {
        max_3(w, x, y)
    } else {
        max_3(x, y, z)
    }
}

/// Returns the minimum of four f64 values
pub fn min_4(w: f64, x: f64, y: f64, z: f64) -> f64 {
    if w < x {
        min_3(w, y, z)
    } else if x < y {
        min_3(w, x, z)
    } else if y < z {
        min_3(w, x, y)
    } else {
        min_3(x, y, z)
    }
}

/// Applies a 1D kernel to `pixels`
pub fn apply_1d_kernel(pixels: SubImage<f64>, kernel: &[f64]) -> ImgProcResult<Vec<f64>> {
    let size = pixels.info().size() as usize;
    let channels = pixels.info().channels as usize;

    error::check_odd(kernel.len(), "kernel length")?;
    error::check_equal(kernel.len(), size, "pixels and kernel dimensions")?;

    let mut vec = vec![0.0; channels];

    // Apply kernel
    for i in 0..size {
        for j in 0..channels {
            vec[j] += kernel[i] * pixels[i][j];
        }
    }

    Ok(vec)
}

/// Applies a 2D kernel to `pixels`
pub fn apply_2d_kernel(pixels: SubImage<f64>, kernel: &[f64]) -> ImgProcResult<Vec<f64>> {
    let size = pixels.info().width as usize;
    let num_channels = pixels.info().channels as usize;

    error::check_odd(kernel.len(), "kernel length")?;
    error::check_equal(kernel.len(), size, "pixels and kernel dimensions")?;

    let mut vec = vec![0.0; num_channels];

    // Apply kernel
    for y in 0..size {
        for x in 0..size {
            let index = y * size + x;
            for j in 0..num_channels {
                vec[j] += kernel[index] * pixels[index][j];
            }
        }
    }

    Ok(vec)
}

/// Calculates the distance between two points
pub fn distance(x_1: u32, y_1: u32, x_2: u32, y_2: u32) -> f64 {
    let x_dist = (x_1 as f64) - (x_2 as f64);
    let y_dist = (y_1 as f64) - (y_2 as f64);

    ((x_dist * x_dist) + (y_dist * y_dist)).sqrt()
}

/// Generates a matrix of distances relative to the center of the matrix
pub fn generate_spatial_mat(size: u32, spatial: f64) -> ImgProcResult<Vec<f64>> {
    let center = size / 2;
    let mut mat = vec![0.0; (size * size) as usize];

    for y in 0..(center + 1) {
        for x in 0..(center + 1) {
            if mat[(y * size + x) as usize] == 0.0 && !(x == center && y == center) {
                let dist = distance(center, center, x, y);
                let g = gaussian_fn(dist, spatial)?;
                mat[(y * size + x) as usize] = g;

                if x == y {
                    let delta = center - y;
                    let coord = center + delta;

                    mat[(coord * size + x) as usize] = g;
                    mat[(x * size + coord) as usize] = g;
                    mat[(coord * size + coord) as usize] = g;
                } else if x == center {
                    let delta = center - y;

                    mat[(x * size + x - delta) as usize] = g;
                    mat[(x * size + x + delta) as usize] = g;
                    mat[((x + delta) * size + x) as usize] = g;
                } else {
                    let delta_x = center - x;
                    let delta_y = center - y;
                    let pos_x = center + delta_x;
                    let pos_y = center + delta_y;
                    let neg_x = center - delta_x;
                    let neg_y = center - delta_y;

                    mat[(neg_x * size + neg_y) as usize] = g;
                    mat[(neg_y * size + pos_x) as usize] = g;
                    mat[(pos_x * size + neg_y) as usize] = g;
                    mat[(pos_y * size + neg_x) as usize] = g;
                    mat[(neg_x * size + pos_y) as usize] = g;
                    mat[(pos_y * size + pos_x) as usize] = g;
                    mat[(pos_x * size + pos_y) as usize] = g;
                }
            }
        }
    }

    Ok(mat)
}

/// Calculates the Gaussian function for G_sigma(x)
pub fn gaussian_fn(x: f64, sigma: f64) -> ImgProcResult<f64> {
    error::check_non_neg(sigma, "sigma")?;

    let sigma_squared = sigma * sigma;

    Ok((1.0 / (2.0 * PI * sigma_squared)) * E.powf(-(x * x) / (2.0 * sigma_squared)))
}