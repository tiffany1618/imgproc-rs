//! A module for image tone operations

use crate::util;
use crate::util::enums::{Tone, White};
use crate::core::colorspace;
use crate::image::Image;
use crate::error::{ImgProcError, ImgProcResult};

use std::collections::HashMap;

/// Adjusts brightness by adding `bias` to each RGB channel if `method` is `Tone::Rgb` or adding
/// `bias` to the luminance value (Y) of `input` in CIE XYZ if `method` is `Tone::Xyz`
pub fn brightness(input: &Image<u8>, bias: i32, method: Tone) -> ImgProcResult<Image<u8>> {
    if bias < 0 || bias > 255 {
        return Err(ImgProcError::InvalidArgError("bias is not in range 0 to 255".to_string()));
    }

    match method {
        Tone::Rgb => {
            let mut lookup_table: [u8; 256] = [0; 256];
            util::create_lookup_table(&mut lookup_table, |i| {
            (i as i32 + bias) as u8
            });

            Ok(input.map_channels_if_alpha(|channel| lookup_table[channel as usize], |a| a))
        },
        Tone::Xyz => {
            let mut xyz = colorspace::srgb_to_xyz(input);
            xyz.edit_channel(|num| num + (bias as f64 / 255.0), 1);
            Ok(colorspace::xyz_to_srgb(&xyz))
        },
    }
}

/// Adjusts contrast by multiplying each RGB channel by `gain` if `method` is `Tone::Rgb` or
/// multiplying the luminance value (Y) of `input` in CIE XYZ by `gain` if `method` is `Tone::Xyz`
// gain > 0
pub fn contrast(input: &Image<u8>, gain: f64, method: Tone) -> ImgProcResult<Image<u8>> {
    if gain <= 0.0 {
        return Err(ImgProcError::InvalidArgError("gain is negative".to_string()));
    }

    match method {
        Tone::Rgb => {
            let mut lookup_table: [u8; 256] = [0; 256];
            util::create_lookup_table(&mut lookup_table, |i| {
                (i as f64 * gain).round() as u8
            });

            Ok(input.map_channels_if_alpha(|channel| lookup_table[channel as usize], |a| a))
        },
        Tone::Xyz => {
            let mut xyz = colorspace::srgb_to_xyz(input);
            xyz.edit_channel(|num| num * gain, 1);
            Ok(colorspace::xyz_to_srgb(&xyz))
        },
    }
}

/// Adjusts saturation by adding `saturation` to the saturation value (S) of `input` in HSV
pub fn saturation(input: &Image<u8>, saturation: i32) -> ImgProcResult<Image<u8>> {
    let mut hsv = colorspace::rgb_to_hsv(input);
    hsv.edit_channel(|s| (s + (saturation as f64 / 255.0)) as f64, 1);

    Ok(colorspace::hsv_to_rgb(&hsv))
}

/// Performs a histogram equalization on `input`
///
/// # Arguments
///
/// * `alpha` - Represents the amount of equalization, where 0 corresponds to no equalization and
/// 1 corresponds to full equalization
/// * `ref_white` - A string slice representing the reference white value of the image
/// * `precision` - See the function `util::generate_histogram_percentiles`
pub fn histogram_equalization(input: &Image<u8>, alpha: f64, ref_white: &White, precision: f64) -> ImgProcResult<Image<u8>> {
    if alpha < 0.0 || alpha > 1.0 {
        return Err(ImgProcError::InvalidArgError("alpha is not in range 0 to 1".to_string()));
    } else if precision <= 0.0 {
        return Err(ImgProcError::InvalidArgError("precision is not positive".to_string()));
    }

    let mut lab = colorspace::srgb_to_lab(input, ref_white);
    let mut percentiles = HashMap::new();
    util::generate_histogram_percentiles(&lab, &mut percentiles, precision);

    lab.edit_channel(|num| {
        let key = (num * precision).round() as i32;
        (alpha * percentiles.get(&key).unwrap() * 100.0) + ((1.0 - alpha) * num)
    }, 0);

    Ok(colorspace::lab_to_srgb(&lab, ref_white))
}
