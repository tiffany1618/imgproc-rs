//! A module for image utility functions

use std::collections::{BTreeMap, HashMap};

use crate::enums::White;
use crate::error::{ImgProcResult, ImgProcError};
use crate::image::{BaseImage, Image, Number};
use std::f64::consts::{PI, E};

pub mod math;
pub mod constants;

////////////////////////////
// Image helper functions
////////////////////////////

/// Returns a tuple representing the XYZ tristimulus values for a given reference white value
pub fn generate_xyz_tristimulus_vals(ref_white: &White) -> ImgProcResult<(f64, f64, f64)> {
    return match ref_white {
        White::D50 => Ok((96.4212, 100.0, 82.5188)),
        White::D65 => Ok((95.0489, 100.0, 103.8840)),
    }
}

/// A helper function for the colorspace conversion from CIE XYZ to CIELAB
pub fn xyz_to_lab_fn(num: f64) -> f64 {
    let d: f64 = 6.0 / 29.0;

    if num > d.powf(3.0) {
        num.powf(1.0 / 3.0)
    } else {
        (num / (3.0 * d * d)) + (4.0 / 29.0)
    }
}

/// A helper function for the colorspace conversion from CIELAB to CIE XYZ
pub fn lab_to_xyz_fn(num: f64) -> f64 {
    let d: f64 = 6.0 / 29.0;

    if num > d {
        num.powf(3.0)
    } else {
        3.0 * d * d * (num - (4.0 / 29.0))
    }
}

/// A helper function for histogram equalization
///
/// # Arguments
///
/// * `input` - a reference to a CIELAB `Image`
/// * `percentiles` - a mutable `HashMap` reference relating an L channel intensity to the number
/// of times it occurs in `input` as a percentile
/// * `precision` - The range of possible L channel intensity values (used to convert the intensity
/// value to an i32, which can be used as a key in `HashMap` and `BTreeMap`)
pub fn generate_histogram_percentiles(input: &Image<f64>, percentiles: &mut HashMap<i32, f64>, precision: f64) {
    let mut histogram = BTreeMap::new();

    for y in 0..(input.info().height) {
        for x in 0..(input.info().width) {
            let p = (input.get_pixel(x, y)[0] * precision).round() as i32;
            let count = histogram.entry(p).or_insert(1);
            *count += 1;
        }
    }

    let mut sum: i32 = 0;
    let num_pixels = input.info().size() as f64;
    for (key, val) in &histogram {
        sum += val;
        percentiles.insert(*key, sum as f64 / num_pixels);
    }
}

/// Populates `table` with the appropriate values based on function `f`
pub fn create_lookup_table<T: Number, F>(table: &mut [T; 256], f: F)
    where F: Fn(u8) -> T {
    for i in 0..256 {
        table[i] = f(i as u8);
    }
}

/// Generates a Gaussian kernel
pub fn generate_gaussian_kernel(size: u32, std_dev: f64) -> ImgProcResult<Vec<f64>> {
    if size % 2 == 0 {
        return Err(ImgProcError::InvalidArgError("size is not odd".to_string()));
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

/// Generates a summed-area table in the format of another `Image` of the same type and dimensions
/// as `input`
pub fn summed_area_table(input: &Image<f64>) -> Image<f64> {
    let mut output = Image::blank(input.info());
    let (width, height, channels) = input.info().whc();
    let zeroes = vec![0.0; channels as usize];

    for y in 0..height {
        for x in 0..width {
            let p_in = input.get_pixel(x, y);
            let mut p_out = Vec::new();
            let mut p_top = zeroes.as_slice();
            let mut p_left = zeroes.as_slice();
            let mut p_diag = zeroes.as_slice();

            if x > 0 {
                p_left = output.get_pixel(x - 1, y);

                if y > 0 {
                    p_diag = output.get_pixel(x - 1, y - 1);
                }
            }
            if y > 0 {
                p_top = output.get_pixel(x, y - 1);
            }

            for c in 0..(channels as usize) {
                p_out.push(p_in[c] + p_top[c] + p_left[c] - p_diag[c]);
            }

            output.set_pixel(x, y, &p_out);
        }
    }

    output
}

/// Computes the sum of pixel intensities over a rectangular region with top left corner located
/// at `(x_0, y_0)` and bottom right corner located at `(x_1, y_1)`
pub fn rectangular_intensity_sum(summed_area_table: &Image<f64>, x_0: u32, y_0: u32, x_1: u32, y_1: u32) -> Vec<f64> {
    let channels = summed_area_table.info().channels as usize;
    let mut sum = Vec::new();

    let zeroes = vec![0.0; channels];
    let mut top_left = zeroes.as_slice();
    let mut top_right = zeroes.as_slice();
    let mut bot_left = zeroes.as_slice();
    let bot_right = summed_area_table.get_pixel(x_1, y_1);

    if x_0 != 0 {
        bot_left = summed_area_table.get_pixel(x_0 - 1, y_1);

        if y_0 != 0 {
            top_left = summed_area_table.get_pixel(x_0 - 1, y_0 - 1);
        }
    }
    if y_0 != 0 {
        top_right = summed_area_table.get_pixel(x_1, y_0 - 1);
    }

    for i in 0..channels {
        sum.push(bot_right[i] - top_right[i] - bot_left[i] + top_left[i]);
    }

    sum
}

//////////
// Misc.
//////////

/// Returns `true` if an image is a grayscale image
pub fn is_grayscale(channels: u8, alpha: bool) -> bool {
    (alpha && channels == 2) || (!alpha && channels == 1)
}