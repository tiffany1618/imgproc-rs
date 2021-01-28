use crate::util;
use crate::util::math;
use crate::util::constant::{GAMMA, SRGB_TO_XYZ_MAT, XYZ_TO_SRGB_MAT};
use crate::image::Image;

use std::cmp;

/// Converts an image from RGB to Grayscale
pub fn rgb_to_grayscale(input: &Image<u8>) -> Image<u8> {
    input.map_pixels_if_alpha(|channels| {
        let mut sum = 0.0;
        for channel in channels.iter() {
            sum += *channel as f64;
        }

        vec![(sum / channels.len() as f64) as u8]
    }, |a| a)
}

/// Converts an f64 image from RGB to Grayscale
pub fn rgb_to_grayscale_f64(input: &Image<f64>) -> Image<f64> {
    input.map_pixels_if_alpha(|channels| {
        let mut sum = 0.0;
        for channel in channels.iter() {
            sum += *channel;
        }

        vec![sum / channels.len() as f64]
    }, |a| a)
}

/// Linearizes an sRGB image
// Input: sRGB range [0, 255]
// Output: sRGB range [0, 1] linearized
pub fn linearize_srgb(input: &Image<u8>) -> Image<f64> {
    let mut lookup_table: [f64; 256] = [0.0; 256];
    util::create_lookup_table(&mut lookup_table, |i| {
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
// Input: sRGB range [0, 1] linearized
// Output: sRGB range [0, 255]
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
// Input: sRGB range [0, 1] linearized
// Output: CIE XYZ range [0, 1]
pub fn srgb_lin_to_xyz(input: &Image<f64>) -> Image<f64> {
    input.map_pixels_if_alpha(|channels| {
        math::vector_mul(&SRGB_TO_XYZ_MAT, channels).unwrap()
    }, |a| a)
}

/// Converts an image from CIE XYZ to linearized sRGB
// Input: CIE XYZ range [0, 1]
// Output: sRGB range [0, 1] linearized
pub fn xyz_to_srgb_lin(input: &Image<f64>) -> Image<f64> {
    input.map_pixels_if_alpha(|channels| {
        math::vector_mul(&XYZ_TO_SRGB_MAT, channels).unwrap()
    }, |a| a)
}

/// Converts an image from CIE XYZ to CIELAB
// Input: CIEXYZ range [0, 1]
// Output: CIELAB with L* channel range [0, 1]
pub fn xyz_to_lab(input: &Image<f64>, ref_white: &str) -> Image<f64> {
    let (x_n, y_n, z_n) = util::generate_xyz_tristimulus_vals(ref_white).unwrap();

    input.map_pixels_if_alpha(|channels| {
        let x = util::xyz_to_lab_fn(channels[0]) * 100.0 / x_n;
        let y = util::xyz_to_lab_fn(channels[1]) * 100.0 / y_n;
        let z = util::xyz_to_lab_fn(channels[2]) * 100.0 / z_n;

        vec![116.0 * y - 16.0,
             500.0 * (x - y),
             200.0 * (y - z)]
    }, |a| a)
}

/// Converts an image from CIELAB to CIE XYZ
// Input: CIELAB with L* channel range [0, 1]
// Output: CIEXYZ range [0, 1]
pub fn lab_to_xyz(input: &Image<f64>, ref_white: &str) -> Image<f64> {
    let (x_n, y_n, z_n) = util::generate_xyz_tristimulus_vals(ref_white).unwrap();

    input.map_pixels_if_alpha(|channels| {
        let n = (channels[0] + 16.0) / 116.0;

        vec![x_n * util::lab_to_xyz_fn(n + channels[1] / 500.0) / 100.0,
             y_n * util::lab_to_xyz_fn(n) / 100.0,
             z_n * util::lab_to_xyz_fn(n - channels[2] / 200.0) / 100.0]
    }, |a| a)
}

/// Converts an image from RGB to HSV
// Input: RGB range [0, 255]
// Output: HSV range [0, 1]
pub fn rgb_to_hsv(input: &Image<u8>) -> Image<f64> {
    input.map_pixels_if_alpha(|channels| {
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

        vec![hue, saturation, (max as f64) / 255.0]
    }, |a| (a as f64) / 255.0)
}

/// Converts an image from HSV to RGB
// Input: HSV range [0, 1]
// Output: RGB range [0, 255]
pub fn hsv_to_rgb(input: &Image<f64>) -> Image<u8> {
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
    }, |a| (a * 255.0).round() as u8)
}

/// Converts an image from sRGB to CIE XYZ
// Input: sRGB range [0, 255] unlinearized
// Output: CIEXYZ range [0, 1]
pub fn srgb_to_xyz(input: &Image<u8>) -> Image<f64> {
    let linearized = linearize_srgb(input);
    srgb_lin_to_xyz(&linearized)
}

/// Converts an image from CIE XYZ to sRGB
// Input: CIEXYZ range [0, 1]
// Output: sRGB range [0, 255] unlinearized
pub fn xyz_to_srgb(input: &Image<f64>) -> Image<u8> {
    let srgb = xyz_to_srgb_lin(input);
    unlinearize_srgb(&srgb)
}

/// Converts an image from sRGB to CIELAB
// Input: sRGB range [0, 255] unlinearized
// Output: CIELAB
pub fn srgb_to_lab(input: &Image<u8>, ref_white: &str) -> Image<f64> {
    let xyz = srgb_to_xyz(input);
    xyz_to_lab(&xyz, ref_white)
}

/// Converts an image from CIELAB to sRGB
// Input: CIELAB
// Output: sRGB range [0, 255] unlinearized
pub fn lab_to_srgb(input: &Image<f64>, ref_white: &str) -> Image<u8> {
    let xyz = lab_to_xyz(input, ref_white);
    xyz_to_srgb(&xyz)
}

impl Image<f64> {
    /// Linearizes an sRGB image
    // Input: sRGB range [0, 255]
    // Output: sRGB range [0, 1] linearized
    pub fn linearize_srgb(&mut self) {
        let mut lookup_table: [f64; 256] = [0.0; 256];
        util::create_lookup_table(&mut lookup_table, |i| {
            let val = i as f64;
            if val <= 10.0 {
                val / 3294.0
            } else {
                ((val + 14.025) / 269.025).powf(GAMMA)
            }
        });

        self.apply_channels_if_alpha(|i| lookup_table[i as usize], |a| a as f64)
    }

    /// "Unlinearizes" a previously linearized sRGB image
    // Input: sRGB range [0, 1] linearized
    // Output: sRGB range [0, 255]
    pub fn unlinearize_srgb(&mut self) {
        self.apply_channels_if_alpha(|num| {
            if num <= 0.0031308 {
                num * 3294.6
            } else {
                269.025 * num.powf(1.0 / GAMMA) - 14.025
            }
        }, |a| a)
    }

    /// Converts an image from linearized sRGB to CIE XYZ
    // Input: sRGB range [0, 1] linearized
    // Output: CIE XYZ range [0, 1]
    pub fn srgb_lin_to_xyz(&mut self) {
        self.apply_pixels_if_alpha(|channels| {
            math::vector_mul(&SRGB_TO_XYZ_MAT, channels).unwrap()
        }, |a| a)
    }

    /// Converts an image from CIE XYZ to linearized sRGB
    // Input: CIE XYZ range [0, 1]
    // Output: sRGB range [0, 1] linearized
    pub fn xyz_to_srgb_lin(&mut self) {
        self.apply_pixels_if_alpha(|channels| {
            math::vector_mul(&XYZ_TO_SRGB_MAT, channels).unwrap()
        }, |a| a)
    }

    /// Converts an image from CIE XYZ to CIELAB
    // Input: CIEXYZ range [0, 1]
    // Output: CIELAB with L* channel range [0, 1]
    pub fn xyz_to_lab(&mut self, ref_white: &str) {
        let (x_n, y_n, z_n) = util::generate_xyz_tristimulus_vals(ref_white).unwrap();

        self.apply_pixels_if_alpha(|channels| {
            let x = util::xyz_to_lab_fn(channels[0]) * 100.0 / x_n;
            let y = util::xyz_to_lab_fn(channels[1]) * 100.0 / y_n;
            let z = util::xyz_to_lab_fn(channels[2]) * 100.0 / z_n;

            vec![116.0 * y - 16.0,
                 500.0 * (x - y),
                 200.0 * (y - z)]
        }, |a| a)
    }

    /// Converts an image from CIELAB to CIE XYZ
    // Input: CIELAB with L* channel range [0, 1]
    // Output: CIEXYZ range [0, 1]
    pub fn lab_to_xyz(&mut self, ref_white: &str) {
        let (x_n, y_n, z_n) = util::generate_xyz_tristimulus_vals(ref_white).unwrap();

        self.apply_pixels_if_alpha(|channels| {
            let n = (channels[0] + 16.0) / 116.0;

            vec![x_n * util::lab_to_xyz_fn(n + channels[1] / 500.0) / 100.0,
                 y_n * util::lab_to_xyz_fn(n) / 100.0,
                 z_n * util::lab_to_xyz_fn(n - channels[2] / 200.0) / 100.0]
        }, |a| a)
    }

    /// Converts an image from RGB to HSV
    // Input: RGB range [0, 255]
    // Output: HSV range [0, 1]
    pub fn rgb_to_hsv(&mut self) {
        self.apply_pixels_if_alpha(|channels| {
            let max = math::max(channels[0], channels[1], channels[2]);
            let min = math::min(channels[0], channels[1], channels[2]);
            let range = (max - min) / 255.0;

            let r = channels[0] / 255.0;
            let g = channels[1] / 255.0;
            let b = channels[2] / 255.0;

            let mut saturation: f64 = 0.0;
            if max != 0.0 { saturation = range / (max as f64 / 255.0); }

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

            vec![hue, saturation, (max as f64) / 255.0]
        }, |a| a / 255.0)
    }

    /// Converts an image from HSV to RGB
    // Input: HSV range [0, 1]
    // Output: RGB range [0, 255]
    pub fn hsv_to_rgb(&mut self) {
        self.apply_pixels_if_alpha(|channels| {
            if channels[1] == 0.0 {
                let val = channels[2] * 255.0;
                return vec![val, val, val];
            }

            let hue = channels[0] * 6.0;
            let f = hue - hue.floor();
            let p = channels[2] * (1.0 - channels[1]) * 255.0;
            let q = channels[2] * (1.0 - channels[1] * f) * 255.0;
            let t = channels[2] * (1.0 - channels[1] * (1.0 - f)) * 255.0;
            let val = channels[2] * 255.0;

            match hue.floor() as u8 {
                0 => vec![val, t, p],
                1 => vec![q, val, p],
                2 => vec![p, val, t],
                3 => vec![p, q, val],
                4 => vec![t, p, val],
                _ => vec![val, p, q],
            }
        }, |a| a * 255.0)
    }

    /// Converts an image from sRGB to CIE XYZ
    // Input: sRGB range [0, 255] unlinearized
    // Output: CIEXYZ range [0, 1]
    pub fn srgb_to_xyz(&mut self) {
        self.linearize_srgb();
        self.srgb_lin_to_xyz();
    }

    /// Converts an image from CIE XYZ to sRGB
    // Input: CIEXYZ range [0, 1]
    // Output: sRGB range [0, 255] unlinearized
    pub fn xyz_to_srgb(&mut self) {
        self.xyz_to_srgb_lin();
        self.unlinearize_srgb();
    }

    /// Converts an image from sRGB to CIELAB
    // Input: sRGB range [0, 255] unlinearized
    // Output: CIELAB
    pub fn srgb_to_lab(&mut self, ref_white: &str) {
        self.srgb_to_xyz();
        self.xyz_to_lab(ref_white);
    }

    /// Converts an image from CIELAB to sRGB
    // Input: CIELAB
    // Output: sRGB range [0, 255] unlinearized
    pub fn lab_to_srgb(&mut self, ref_white: &str) {
        self.lab_to_xyz(ref_white);
        self.xyz_to_srgb();
    }
}