//! A module for image colorspace conversion operations

use std::cmp;

use crate::enums::White;
use crate::image::{BaseImage, Image};
use crate::util;
use crate::util::constants::{GAMMA, SRGB_TO_XYZ_MAT, XYZ_TO_SRGB_MAT};

#[cfg(feature = "simd")]
use crate::simd;

/// Converts an image from RGB to Grayscale
pub fn rgb_to_grayscale(input: &Image<u8>) -> Image<u8> {
    #[cfg(feature = "simd")]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe { simd::average_rgb_256_u8(input) }
        } else {
            Image::blank(input.info())
        }
    }

    #[cfg(not(feature = "simd"))]
    {
        input.map_pixels_if_alpha(|channels, p_out| {
            let mut sum = 0.0;
            for channel in channels.iter() {
                sum += *channel as f64;
            }

            p_out.push((sum / channels.len() as f64) as u8);
        }, |a| a)
    }
}

/// Converts an f64 image from RGB to Grayscale
pub fn rgb_to_grayscale_f64(input: &Image<f64>) -> Image<f64> {
    input.map_pixels_if_alpha(|channels, p_out| {
        let mut sum = 0.0;
        for channel in channels.iter() {
            sum += *channel;
        }

        p_out.push(sum / channels.len() as f64);
    }, |a| a)
}

/// Linearizes an sRGB image
///
/// * Input: sRGB image with channels in range [0, 255]
/// * Output: linearized sRGB image with channels in range [0, 1]
pub fn linearize_srgb(input: &Image<u8>) -> Image<f64> {
    let mut lookup_table: [f64; 256] = [0.0; 256];
    util::generate_lookup_table(&mut lookup_table, |i| {
        let val = i as f64;
        if val <= 10.0 {
            val / 3294.0
        } else {
            ((val + 14.025) / 269.025).powf(GAMMA)
        }
    });

    input.map_channels_if_alpha(|i| lookup_table[i as usize], |a| a as f64)
}

/// "Unlinearizes" a previously linearized sRGB image
///
/// * Input: linearized sRGB image with channels in range [0, 1]
/// * Output: sRGB image with channels in range [0, 255]
pub fn unlinearize_srgb(input: &Image<f64>) -> Image<u8> {
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
/// * Input: linearized sRGB image with channels in range [0, 1]
/// * Output: CIE XYZ image with channels in range [0, 1]
pub fn srgb_lin_to_xyz(input: &Image<f64>) -> Image<f64> {
    input.map_pixels_if_alpha(|channels, p_out| {
        util::vector_mul_mut(&SRGB_TO_XYZ_MAT, channels, p_out).unwrap()
    }, |a| a)
}

/// Converts an image from CIE XYZ to linearized sRGB
///
/// * Input: CIE XYZ image with channels in range [0, 1]
/// * Output: linearized sRGB image with channels in range [0, 1]
pub fn xyz_to_srgb_lin(input: &Image<f64>) -> Image<f64> {
    input.map_pixels_if_alpha(|channels, p_out| {
        util::vector_mul_mut(&XYZ_TO_SRGB_MAT, channels, p_out).unwrap()
    }, |a| a)
}

/// Converts an image from CIE XYZ to CIELAB
///
/// * Input: CIE XYZ image with channels in range [0, 1]
/// * Output: CIELAB image with L* channel range [0, 100] and a*, b* channels range [-128, 127]
pub fn xyz_to_lab(input: &Image<f64>, ref_white: &White) -> Image<f64> {
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
/// * Input: CIELAB image with L* channel range [0, 100] and a*, b* channels range [-128, 127]
/// * Output: CIE XYZ image with channels in range [0, 1]
pub fn lab_to_xyz(input: &Image<f64>, ref_white: &White) -> Image<f64> {
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
/// * Input: RGB image with channels in range [0, 255]
/// * Output: HSV image with channels in range [0, 1]
pub fn rgb_to_hsv(input: &Image<u8>) -> Image<f64> {
    input.map_pixels_if_alpha(|channels, p_out| {
        let max: u8 = cmp::max(cmp::max(channels[0], channels[1]), channels[2]);
        let min: u8 = cmp::min(cmp::min(channels[0], channels[1]), channels[2]);
        let range = (max - min) as f64 / 255.0;

        let r = channels[0] as f64 / 255.0;
        let g = channels[1] as f64 / 255.0;
        let b = channels[2] as f64 / 255.0;

        let mut saturation: f64 = 0.0;
        if max != 0 { saturation = range / (max as f64 / 255.0); }

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
        } else if hue > 6.0 {
            hue -= 1.0;
        }

        p_out.extend([hue, saturation, (max as f64) / 255.0].iter());
    }, |a| (a as f64) / 255.0)
}

/// Converts an image from HSV to RGB
///
/// * Input: HSV image with channels in range [0, 1]
/// * Output: RGB image with channels in range [0, 255]
pub fn hsv_to_rgb(input: &Image<f64>) -> Image<u8> {
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
/// * Input: sRGB image with channels in range [0, 255]
/// * Output: CIE XYZ image with channels in range [0, 1]
pub fn srgb_to_xyz(input: &Image<u8>) -> Image<f64> {
    let linearized = linearize_srgb(input);
    srgb_lin_to_xyz(&linearized)
}

/// Converts an image from CIE XYZ to sRGB
///
/// * Input: CIE XYZ image with channels in range [0, 1]
/// * Output: sRGB image with channels in range [0, 255]
pub fn xyz_to_srgb(input: &Image<f64>) -> Image<u8> {
    let srgb = xyz_to_srgb_lin(input);
    unlinearize_srgb(&srgb)
}

/// Converts an image from sRGB to CIELAB
///
/// * Input: sRGB image with channels in range [0, 255]
/// * Output: CIELAB image with L* channel range [0, 100] and a*, b* channels range [-128, 127]
pub fn srgb_to_lab(input: &Image<u8>, ref_white: &White) -> Image<f64> {
    let xyz = srgb_to_xyz(input);
    xyz_to_lab(&xyz, ref_white)
}

/// Converts an image from CIELAB to sRGB
///
/// * Input: CIELAB image with L* channel range [0, 100] and a*, b* channels range [-128,127]
/// * Output: sRGB image with channels in range [0, 255]
pub fn lab_to_srgb(input: &Image<f64>, ref_white: &White) -> Image<u8> {
    let xyz = lab_to_xyz(input, ref_white);
    xyz_to_srgb(&xyz)
}
