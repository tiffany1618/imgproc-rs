//! A module for image channel type conversions

use crate::image::Image;
use crate::error::ImgProcResult;

/// Scales channels from range `current_min` to `current_max` to range `scaled_min` to `scaled_max`
pub fn scale_channels(input: &Image<f64>, current_min: f64, scaled_min: f64, current_max: f64, scaled_max: f64) -> ImgProcResult<Image<f64>> {
    Ok(input.map_channels(|channel| {
        (channel - current_min) / (current_max - current_min) * (scaled_max - scaled_min) + scaled_min
    }))
}

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