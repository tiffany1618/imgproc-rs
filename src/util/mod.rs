//! A module for image utility functions

pub mod math;
pub mod constants;
pub mod enums;

use crate::image::{Image, BaseImage, Number};
use crate::error::{ImgProcResult};
use crate::util::enums::White;

use std::collections::{HashMap, BTreeMap};

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

/// Converts an `Image<f64>` to an `Image<u8>`
pub fn image_f64_to_u8(input: &Image<f64>) -> Image<u8> {
    input.map_channels(|channel| channel.round() as u8)
}

/// Converts an `Image<f64>` with channels in range 0 to `scale` to an `Image<u8>` with channels
/// in range 0 to 255
pub fn image_f64_to_u8_scale(input: &Image<f64>, scale: u32) -> Image<u8> {
    input.map_channels(|channel| (channel / scale as f64 * 255.0).round() as u8)
}

/// Converts an `Image<u8>` to an `Image<f64>`
pub fn image_u8_to_f64(input: &Image<u8>) -> Image<f64> {
    input.map_channels(|channel| channel as f64)
}

/// Converts an `Image<u8>` to with channels in range 0 to 255 to an `Image<f64>` with channels
/// in range 0 to `scale`
pub fn image_u8_to_f64_scale(input: &Image<u8>, scale: u32) -> Image<f64> {
    input.map_channels(|channel| ((channel as f64 / 255.0) * scale as f64))
}

/// Returns `true` if an image is a grayscale image
pub fn is_grayscale(channels: u8, alpha: bool) -> bool {
    (alpha && channels == 2) || (!alpha && channels == 1)
}