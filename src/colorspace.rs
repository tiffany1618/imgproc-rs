//! A module for image colorspace conversion operations

use std::cmp;

use crate::enums::White;
use crate::image::Image;
use crate::util;
use crate::util::constants::{GAMMA, SRGB_TO_XYZ_MAT, XYZ_TO_SRGB_MAT};

#[cfg(feature = "simd")]
use crate::simd;

/// Converts a u8 image from RGB to Grayscale
pub fn rgb_to_grayscale(input: &Image<u8>) -> Image<u8> {
    #[cfg(feature = "simd")]
    {
        if is_x86_feature_detected!("avx2") {
            simd::avg_checked_256_u8(input)
        } else {
            rgb_to_grayscale_norm(input)
        }
    }

    #[cfg(not(feature = "simd"))]
    rgb_to_grayscale_norm(input)
}

fn rgb_to_grayscale_norm(input: &Image<u8>) -> Image<u8> {
    input.map_pixels_if_alpha(|channels, p_out| {
        let mut sum = 0;
        for channel in channels.iter() {
            sum += *channel as i16;
        }

        p_out.push((sum / channels.len() as i16) as u8);
    }, |a| a)
}

/// Converts an f32 image from RGB to Grayscale
pub fn rgb_to_grayscale_f32(input: &Image<f32>) -> Image<f32> {
    input.map_pixels_if_alpha(|channels, p_out| {
        let mut sum = 0.0;
        for channel in channels.iter() {
            sum += *channel;
        }

        p_out.push(sum / channels.len() as f32);
    }, |a| a)
}

/// Linearizes an sRGB image
///
/// * Input: u8 sRGB image with channels in range [0, 255]
/// * Output: linearized f32 sRGB image with channels in range [0, 1]
pub fn linearize_srgb_f32(input: &Image<u8>) -> Image<f32> {
    let mut lookup_table: [f32; 256] = [0.0; 256];
    util::generate_lookup_table(&mut lookup_table, |i| {
        let val = i as f32;
        if val <= 10.0 {
            val / 3294.0
        } else {
            ((val + 14.025) / 269.025).powf(GAMMA)
        }
    });

    input.map_channels_if_alpha(|i| lookup_table[i as usize], |a| a as f32)
}

/// "Unlinearizes" a previously linearized sRGB image
///
/// * Input: f32 linearized sRGB image with channels in range [0, 1]
/// * Output: u8 sRGB image with channels in range [0, 255]
pub fn unlinearize_srgb_f32(input: &Image<f32>) -> Image<u8> {
    input.map_channels_if_alpha(|num| {
        if num <= 0.0031308 {
            (num * 3294.6) as u8
        } else {
            (269.025 * num.powf(1.0 / GAMMA) - 14.025) as u8
        }
    }, |a| a.round() as u8)
}

/// Converts an image from linearized sRGB to CIE XYZ
///
/// * Input: f32 linearized sRGB image with channels in range [0, 1]
/// * Output: f32 CIE XYZ image with channels in range [0, 1]
pub fn srgb_lin_to_xyz_f32(input: &Image<f32>) -> Image<f32> {
    input.map_pixels_if_alpha(|channels, p_out| {
        util::vector_mul_mut(&SRGB_TO_XYZ_MAT, channels, p_out).unwrap()
    }, |a| a)
}

/// Converts an image from CIE XYZ to linearized sRGB
///
/// * Input: f32 CIE XYZ image with channels in range [0, 1]
/// * Output: f32 linearized sRGB image with channels in range [0, 1]
pub fn xyz_to_srgb_lin_f32(input: &Image<f32>) -> Image<f32> {
    input.map_pixels_if_alpha(|channels, p_out| {
        util::vector_mul_mut(&XYZ_TO_SRGB_MAT, channels, p_out).unwrap()
    }, |a| a)
}

/// Converts an image from CIE XYZ to CIELAB
///
/// * Input: f32 CIE XYZ image with channels in range [0, 1]
/// * Output: f32 CIELAB image with L* channel range [0, 100] and a*, b* channels range [-128, 127]
pub fn xyz_to_lab_f32(input: &Image<f32>, ref_white: &White) -> Image<f32> {
    let (x_n, y_n, z_n) = util::xyz_tristimulus_vals(ref_white);

    input.map_pixels_if_alpha(|channels, p_out| {
        let x = util::xyz_to_lab_fn(channels[0] * 100.0 / x_n);
        let y = util::xyz_to_lab_fn(channels[1] * 100.0 / y_n);
        let z = util::xyz_to_lab_fn(channels[2] * 100.0 / z_n);

        p_out.extend([116.0 * y - 16.0,
                           500.0 * (x - y),
                           200.0 * (y - z)].iter());
    }, |a| a)
}

/// Converts an image from CIELAB to CIE XYZ
///
/// * Input: f32 CIELAB image with L* channel range [0, 100] and a*, b* channels range [-128, 127]
/// * Output: f32 CIE XYZ image with channels in range [0, 1]
pub fn lab_to_xyz_f32(input: &Image<f32>, ref_white: &White) -> Image<f32> {
    let (x_n, y_n, z_n) = util::xyz_tristimulus_vals(ref_white);

    input.map_pixels_if_alpha(|channels, p_out| {
        let n = (channels[0] + 16.0) / 116.0;

        p_out.extend([x_n * util::lab_to_xyz_fn(n + channels[1] / 500.0) / 100.0,
                           y_n * util::lab_to_xyz_fn(n) / 100.0,
                           z_n * util::lab_to_xyz_fn(n - channels[2] / 200.0) / 100.0].iter());
    }, |a| a)
}

/// Converts an image from RGB to HSV
///
/// * Input: u8 RGB image with channels in range [0, 255]
/// * Output: u8 HSV image with channels in range [0, 255]
pub fn rgb_to_hsv(input: &Image<u8>) -> Image<u8> {
    input.map_pixels_if_alpha(|channels, p_out| {
        let max = cmp::max(cmp::max(channels[0], channels[1]), channels[2]) as i16;
        let min = cmp::min(cmp::min(channels[0], channels[1]), channels[2]) as i16;
        let range = max - min;

        let r = channels[0] as i16;
        let g = channels[1] as i16;
        let b = channels[2] as i16;

        let mut saturation = 0;
        if max != 0 {
            saturation = 255 * range / max;
        }

        let mut hue: i16 = 0;
        if range != 0 {
            if max == r {
                hue = 43 * (g - b) / range;
            } else if max == g {
                hue = 85 + 43 * (b - r) / range;
            } else {
                hue = 170 + 43 * (r - g) / range;
            }
        }

        if hue < 0 {
            hue += 255;
        } else if hue > 255 {
            hue -= 255;
        }

        p_out.extend([hue as u8, saturation as u8, max as u8].iter());
    }, |a| a)
}

/// Converts an image from HSV to RGB
///
/// * Input: u8 HSV image with channels in range [0, 255]
/// * Output: u8 RGB image with channels in range [0, 255]
pub fn hsv_to_rgb(input: &Image<u8>) -> Image<u8> {
    input.map_pixels_if_alpha(|channels, p_out| {
        if channels[1] == 0 {
            let val = channels[2];
            p_out.extend([val, val, val].iter());
            return;
        }

        let hue = channels[0] as i16 / 43;
        let f = (hue - (channels[0] as i16 * 43)) * 6;
        let p = ((channels[2] as i16 * (255 - channels[1] as i16)) / 255) as u8;
        let q = ((channels[2] as i16 * (255 - (channels[1] as i16 * f) / 255)) / 255) as u8;
        let t = ((channels[2] as i16 * (255 - (channels[1] as i16 * (255 - f)) / 255)) / 255) as u8;
        let val = channels[2];

        match hue as u8 {
            0 => p_out.extend([val, t, p].iter()),
            1 => p_out.extend([q, val, p].iter()),
            2 => p_out.extend([p, val, t].iter()),
            3 => p_out.extend([p, q, val].iter()),
            4 => p_out.extend([t, p, val].iter()),
            _ => p_out.extend([val, p, q].iter()),
        }
    }, |a| a)
}

/// Converts an image from RGB to HSV
///
/// * Input: u8 RGB image with channels in range [0, 255]
/// * Output: f32 HSV image with channels in range [0, 1]
pub fn rgb_to_hsv_f32(input: &Image<u8>) -> Image<f32> {
    input.map_pixels_if_alpha(|channels, p_out| {
        let max: u8 = cmp::max(cmp::max(channels[0], channels[1]), channels[2]);
        let min: u8 = cmp::min(cmp::min(channels[0], channels[1]), channels[2]);
        let range = (max - min) as f32 / 255.0;

        let r = channels[0] as f32 / 255.0;
        let g = channels[1] as f32 / 255.0;
        let b = channels[2] as f32 / 255.0;

        let mut saturation: f32 = 0.0;
        if max != 0 { saturation = range / (max as f32 / 255.0); }

        let mut hue = 0.0;
        if range != 0.0 {
            if max == channels[0] {
                hue = (g - b) / range
            } else if max == channels[1] {
                hue = (b - r) / range + 2.0
            } else {
                hue = (r - g) / range + 4.0
            }
        }

        hue /= 6.0;
        if hue < 0.0 {
            hue += 1.0;
        } else if hue > 1.0 {
            hue -= 1.0;
        }

        p_out.extend([hue, saturation, (max as f32) / 255.0].iter());
    }, |a| (a as f32) / 255.0)
}

/// Converts an image from HSV to RGB
///
/// * Input: f32 HSV image with channels in range [0, 1]
/// * Output: u8 RGB image with channels in range [0, 255]
pub fn hsv_to_rgb_f32(input: &Image<f32>) -> Image<u8> {
    input.map_pixels_if_alpha(|channels, p_out| {
        if channels[1] == 0.0 {
            let val = (channels[2] * 255.0) as u8;

            p_out.extend([val, val, val].iter());
            return;
        }

        let hue = channels[0] * 6.0;
        let f = hue - hue.floor();
        let p = (channels[2] * (1.0 - channels[1]) * 255.0) as u8;
        let q = (channels[2] * (1.0 - channels[1] * f) * 255.0) as u8;
        let t = (channels[2] * (1.0 - channels[1] * (1.0 - f)) * 255.0) as u8;
        let val = (channels[2] * 255.0) as u8;

        match hue.floor() as u8 {
            0 => p_out.extend([val, t, p].iter()),
            1 => p_out.extend([q, val, p].iter()),
            2 => p_out.extend([p, val, t].iter()),
            3 => p_out.extend([p, q, val].iter()),
            4 => p_out.extend([t, p, val].iter()),
            _ => p_out.extend([val, p, q].iter()),
        }
    }, |a| (a * 255.0).round() as u8)
}

/// Converts an image from sRGB to CIE XYZ
///
/// * Input: u8 sRGB image with channels in range [0, 255]
/// * Output: f32 CIE XYZ image with channels in range [0, 1]
pub fn srgb_to_xyz_f32(input: &Image<u8>) -> Image<f32> {
    let linearized = linearize_srgb_f32(input);
    srgb_lin_to_xyz_f32(&linearized)
}

/// Converts an image from CIE XYZ to sRGB
///
/// * Input: f32 CIE XYZ image with channels in range [0, 1]
/// * Output: u8 sRGB image with channels in range [0, 255]
pub fn xyz_to_srgb_f32(input: &Image<f32>) -> Image<u8> {
    let srgb = xyz_to_srgb_lin_f32(input);
    unlinearize_srgb_f32(&srgb)
}

/// Converts an image from sRGB to CIELAB
///
/// * Input: u8 sRGB image with channels in range [0, 255]
/// * Output: f32 CIELAB image with L* channel range [0, 100] and a*, b* channels range [-128, 127]
pub fn srgb_to_lab_f32(input: &Image<u8>, ref_white: &White) -> Image<f32> {
    let xyz = srgb_to_xyz_f32(input);
    xyz_to_lab_f32(&xyz, ref_white)
}

/// Converts an image from CIELAB to sRGB
///
/// * Input: f32 CIELAB image with L* channel range [0, 100] and a*, b* channels range [-128,127]
/// * Output: u8 sRGB image with channels in range [0, 255]
pub fn lab_to_srgb_f32(input: &Image<f32>, ref_white: &White) -> Image<u8> {
    let xyz = lab_to_xyz_f32(input, ref_white);
    xyz_to_srgb_f32(&xyz)
}
