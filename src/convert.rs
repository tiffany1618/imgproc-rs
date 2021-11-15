//! A module for image channel type conversions

use crate::image::Image;
use crate::error::ImgProcResult;

/// Scales channels from range `current_min` to `current_max` to range `scaled_min` to `scaled_max`
pub fn scale_channels(input: &Image<f32>, current_min: f32, scaled_min: f32, current_max: f32, scaled_max: f32) -> ImgProcResult<Image<f32>> {
    Ok(input.map_channels(|channel| {
        (channel - current_min) / (current_max - current_min) * (scaled_max - scaled_min) + scaled_min
    }))
}

/// Converts an `Image<f32>` with channels in range 0 to `scale` to an `Image<u8>` with channels
/// in range 0 to 255
pub fn f32_to_u8_scale(input: &Image<f32>, scale: u32) -> Image<u8> {
    input.map_channels(|channel| (channel / scale as f32 * 255.0).round() as u8)
}

/// Converts an `Image<u8>` to with channels in range 0 to 255 to an `Image<f32>` with channels
/// in range 0 to `scale`
pub fn u8_to_f32_scale(input: &Image<u8>, scale: u32) -> Image<f32> {
    input.map_channels(|channel| ((channel as f32 / 255.0) * scale as f32))
}