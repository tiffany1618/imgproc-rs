//! A module for image tone operations

use crate::{util, colorspace, error};
use crate::enums::{Tone, White};
use crate::image::Image;
use crate::error::ImgProcResult;

#[cfg(feature = "simd")]
use crate::simd;

use std::collections::HashMap;

/// Adjusts brightness by adding `bias` to each RGB channel if `method` is `Tone::Rgb`, or adding
/// `bias` to the L* channel of `input` in CIELAB if `method` is `Tone::Lab`
///
/// # Arguments
///
/// * `bias` - Must be between -255 and 255 (inclusive)
pub fn brightness(input: &Image<u8>, bias: i16, method: Tone) -> ImgProcResult<Image<u8>> {
    error::check_in_range(bias.abs(), -255, 255, "bias")?;

    match method {
        Tone::Rgb => {
            #[cfg(feature = "simd")]
            {
                if is_x86_feature_detected!("avx2") {
                    unsafe { simd::adds_256_u8(input, bias) }
                } else {
                    Ok(brightness_rgb_norm(input, bias))
                }
            }

            #[cfg(not(feature = "simd"))]
            Ok(brightness_rgb_norm(input, bias))
        },
        Tone::Lab => {
            let mut lab = colorspace::srgb_to_lab_f32(input, &White::D50);
            let bias_lab = (bias as f32) / 255.0 * 100.0;
            lab.edit_channel(|num| num + bias_lab, 0);
            Ok(colorspace::lab_to_srgb_f32(&lab, &White::D50))
        },
    }
}

fn brightness_rgb_norm(input: &Image<u8>, bias: i16) -> Image<u8> {
    let mut lookup_table: [u8; 256] = [0; 256];
    util::generate_lookup_table(&mut lookup_table, |i| {
        (i as i16 + bias).clamp(0, 255) as u8
    });

    input.map_channels_if_alpha(|channel| lookup_table[channel as usize], |a| a)
}

/// Adjusts contrast by multiplying each RGB channel by `gain` if `method` is `Tone::Rgb`, or
/// multiplying the L* channel of `input` in CIELAB by `gain` if `method` is `Tone::Lab`
///
/// # Arguments
///
/// * `gain` - Must be between 0 and 1 (inclusive)
pub fn contrast(input: &Image<u8>, gain: f32, method: Tone) -> ImgProcResult<Image<u8>> {
    error::check_non_neg(gain, "gain")?;
    error::check_in_range(gain, 0.0, 1.0, "gain")?;

    match method {
        Tone::Rgb => {
            let mut lookup_table: [u8; 256] = [0; 256];
            util::generate_lookup_table(&mut lookup_table, |i| {
                (i as f32 * gain).round().clamp(0.0, 255.0) as u8
            });

            Ok(input.map_channels_if_alpha(|channel| lookup_table[channel as usize], |a| a))
        },
        Tone::Lab => {
            let mut lab = colorspace::srgb_to_lab_f32(input, &White::D50);
            lab.edit_channel(|num| num * gain, 0);
            Ok(colorspace::lab_to_srgb_f32(&lab, &White::D50))
        },
    }
}

/// Adjusts saturation by adding `saturation` to the saturation value (S) of `input` in HSV
///
/// # Arguments
///
/// * `saturation` - Must be between -255 and 255 (inclusive)
pub fn saturation(input: &Image<u8>, saturation: i32) -> ImgProcResult<Image<u8>> {
    error::check_in_range(saturation, -255, 255, "saturation")?;

    let mut hsv = colorspace::rgb_to_hsv_f32(input);
    hsv.edit_channel(|s| (s + (saturation as f32 / 255.0)) as f32, 1);

    Ok(colorspace::hsv_to_rgb_f32(&hsv))
}

/// Performs a gamma correction. `max` indicates the maximum allowed pixel value of the image
///
/// # Arguments
///
/// * `gamma` - Must be non-negative
pub fn gamma(input: &Image<u8>, gamma: f32, max: u8) -> ImgProcResult<Image<u8>> {
    error::check_non_neg(gamma, "gamma")?;

    Ok(input.map_channels_if_alpha(|channel| {
        ((channel as f32 / max as f32).powf(gamma) * (max as f32)).round() as u8
    }, |a| a))
}

/// Performs a histogram equalization on `input`
///
/// # Arguments
///
/// * `alpha` - Represents the amount of equalization, where 0 corresponds to no equalization and
/// 1 corresponds to full equalization
/// * `ref_white` - An enum representing the reference white value of the image
/// * `precision` - Must be non-negative. See
/// [`generate_histogram_percentiles`](../util/fn.generate_histogram_percentiles.html) for a
/// complete description
pub fn histogram_equalization(input: &Image<u8>, alpha: f32, ref_white: &White, precision: f32) -> ImgProcResult<Image<u8>> {
    error::check_non_neg(precision, "precision")?;
    error::check_in_range(alpha, 0.0, 1.0, "alpha")?;

    let mut lab = colorspace::srgb_to_lab_f32(input, ref_white);
    let mut percentiles = HashMap::new();
    util::generate_histogram_percentiles(&lab, &mut percentiles, precision);

    lab.edit_channel(|num| {
        let key = (num * precision).round() as i32;
        (alpha * percentiles.get(&key).unwrap() * 100.0) + ((1.0 - alpha) * num)
    }, 0);

    Ok(colorspace::lab_to_srgb_f32(&lab, ref_white))
}
