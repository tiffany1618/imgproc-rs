use std::cmp;

use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;

use crate::util;
use crate::util::Number;
use crate::image::Image;

// TODO: Fix loss of precision by integer division
pub fn rgb_to_grayscale<T: Number>(input: &Image<T>) -> Image<T> {
    input.map_pixels_if_alpha(|channels_in| {
        vec![(channels_in[0] / 3.into()) + (channels_in[1] / 3.into()) + (channels_in[2] / 3.into())]
    }, |a| a)
}

// Input: sRGB range [0, 1] linearized
// Output: CIEXYZ range [0, 1]
pub fn srgb_to_xyz(input: &Image<f32>) -> Image<f32> {
    let trans_mat = Matrix::new(3, 3, util::sRGB_TO_XYZ_MAT.to_vec());

    input.map_pixels_if_alpha(|channels| {
        (&trans_mat * Vector::new(channels.to_vec())).into_vec()
    }, |a| a)
}

// Input: CIEXYZ range [0, 1]
// Output: sRGB range [0, 1] linearized
pub fn xyz_to_srgb(input: &Image<f32>) -> Image<f32> {
    let trans_mat = Matrix::new(3, 3, util::XYZ_TO_sRGB_MAT);

    input.map_pixels_if_alpha(|channels| {
        (&trans_mat * Vector::new(channels.to_vec())).into_vec()
    }, |a| a)
}

// Input: CIEXYZ range [0, 1]
// Output: CIELAB with L* channel range [0, 1]
pub fn xyz_to_lab(input: &Image<f32>, ref_white: &str) -> Image<f32> {
    let (x_n, y_n, z_n) = util::generate_xyz_tristimulus_vals(ref_white).unwrap();

    input.map_pixels_if_alpha(|channels| {
        let x = util::xyz_to_lab_fn(channels[0]) / x_n;
        let y = util::xyz_to_lab_fn(channels[1]) / y_n;
        let z = util::xyz_to_lab_fn(channels[2]) / z_n;

        vec![116.0 * y - 16.0,
             500.0 * (x - y),
             200.0 * (y - z)]
    }, |a| a)
}

// Input: CIELAB with L* channel range [0, 100]
// Output: CIEXYZ range [0, 1]
pub fn lab_to_xyz(input: &Image<f32>, ref_white: &str) -> Image<f32> {
    let (x_n, y_n, z_n) = util::generate_xyz_tristimulus_vals(ref_white).unwrap();

    input.map_pixels_if_alpha(|channels| {
        let n = (channels[0] + 16.0) / 116.0;

        vec![x_n * util::lab_to_xyz_fn(n + channels[1] / 500.0),
             y_n * util::lab_to_xyz_fn(n),
             z_n * util::lab_to_xyz_fn(n - channels[2] / 200.0)]
    }, |a| a)
}

// Input: RGB range [0, 255]
// Output: HSV range [0, 1]
pub fn rgb_to_hsv(input: Image<u8>) -> Image<f32> {
    input.map_pixels_if_alpha(|channels| {
        let max = cmp::max(cmp::max(channels[0], channels[1]), channels[2]) as f32 / 255.0;
        let min = cmp::min(cmp::min(channels[0], channels[1]), channels[2]) as f32 / 255.0;
        let range = max - min;

        let r = channels[0] as f32 / 255.0;
        let g = channels[1] as f32 / 255.0;
        let b = channels[2] as f32 / 255.0;

        let mut saturation: f32 = 0.0;
        let mut hue: f32 = 0.0;

        if max != 0.0 { saturation = range / max; }

        match max {
            r => { hue = (g - b) / range },
            g => { hue = (b - r) / range + 2.0 },
            b => { hue = (r - g) / range + 4.0 },
        }

        vec![hue, saturation, max]
    }, |a| a as f32)
}

// Input: HSV range [0, 1]
// Output: RGB range [0, 255]
pub fn hsv_to_rgb(input: &Image<f32>) -> Image<u8> {
    input.map_pixels_if_alpha(|channels| {
        if channels[1] == 0.0 {
            let val = (channels[2] * 255.0) as u8;
            return vec![val, val, val];
        }

        let hue = channels[0] * 6.0;
        let f = hue - hue.floor();
        let p = (channels[2] * (1.0 - channels[1]) * 255.0) as u8;
        let q = (channels[2] * (1.0 - channels[1] * f) * 255.0) as u8;
        let t = (channels[2] * (1.0 - channels[1] * (1.0 - f)) * 255.0) as u8;
        let val = (channels[2] * 255.0) as u8;

        match hue.floor() as u8 {
            0 => vec![val, t, p],
            1 => vec![q, val, p],
            2 => vec![p, val, t],
            3 => vec![p, q, val],
            4 => vec![t, p, val],
            _ => vec![val, p, q],
        }
    }, |a| a.round() as u8)
}