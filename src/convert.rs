//! A module for image channel type conversions

use crate::image::Image;

/// Converts an `Image<f64>` with channels in range 0 to `scale` to an `Image<u8>` with channels
/// in range 0 to 255
pub fn f64_to_u8_scale(input: &Image<f64>, scale: u32) -> Image<u8> {
    input.map_channels(|channel| (channel / scale as f64 * 255.0).round() as u8)
}

/// Converts an `Image<u8>` to with channels in range 0 to 255 to an `Image<f64>` with channels
/// in range 0 to `scale`
pub fn u8_to_f64_scale(input: &Image<u8>, scale: u32) -> Image<f64> {
    input.map_channels(|channel| ((channel as f64 / 255.0) * scale as f64))
}