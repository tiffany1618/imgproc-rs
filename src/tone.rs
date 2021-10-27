//! A module for image tone operations

use crate::{util, colorspace, error};
use crate::enums::{Tone, White};
use crate::image::{Image, BaseImage};
use crate::error::ImgProcResult;

use std::collections::HashMap;
use std::{f64, mem};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;

/// Adjusts brightness by adding `bias` to each RGB channel if `method` is `Tone::Rgb`, or adding
/// `bias` to the L* channel of `input` in CIELAB if `method` is `Tone::Lab`
///
/// # Arguments
///
/// * `bias` - Must be between 0 and 255 (inclusive)
#[allow(unsafe_code)]
pub fn brightness(input: &Image<u8>, bias: i16, method: Tone) -> ImgProcResult<Image<u8>> {
    error::check_in_range(bias.abs(), 0, 255, "bias")?;

    match method {
        Tone::Rgb => {
            if is_x86_feature_detected!("avx2") {
                unsafe {
                    Ok(brightness_rgb_256(input, bias))
                }
            } else {
                Ok(brightness_rgb(input, bias))
            }
        },
        Tone::Lab => {
            let mut lab = colorspace::srgb_to_lab(input, &White::D50);
            lab.edit_channel(|num| num + (bias as f64) * 255.0 / 100.0, 0);
            Ok(colorspace::lab_to_srgb(&lab, &White::D50))
        },
    }
}

pub fn brightness_norm(input: &Image<u8>, bias: i16) -> Image<u8> {
    brightness_rgb(input, bias)
}

pub fn brightness_256(input: &Image<u8>, bias: i16) -> Image<u8> {
    unsafe { brightness_rgb_256(input, bias) }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn brightness_rgb_256(input: &Image<u8>, bias: i16) -> Image<u8> {
    let num_bytes = input.info().full_size();
    let mut data: Vec<u8> = vec![0; num_bytes as usize];
    let mut data_ptr: *mut u8 = data.as_mut_ptr();
    let input_ptr = input.data().as_ptr();

    let bias_256 = unsafe { _mm256_set1_epi8(bias.abs() as i8) };

    let mut i = 0;
    while i < num_bytes {
        unsafe {
            let chunk = _mm256_loadu_si256(input_ptr.add(i as usize) as *const __m256i);

            let res = if bias > 0 {
                _mm256_adds_epu8(chunk, bias_256)
            } else {
                _mm256_subs_epu8(chunk, bias_256)
            };

            _mm256_storeu_si256(data_ptr.add(i as usize) as *mut __m256i, res);
        }
        i += 32;
    }

    if i != 0 {
        for j in i..num_bytes {
            data.push((input.data()[j as usize] as i16 + bias).clamp(0, 255) as u8);
        }
    }

    Image::from_vec(input.info().width, input.info().height, input.info().channels,
                    input.info().alpha, data)
}

fn brightness_rgb(input: &Image<u8>, bias: i16) -> Image<u8> {
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
pub fn contrast(input: &Image<u8>, gain: f64, method: Tone) -> ImgProcResult<Image<u8>> {
    error::check_non_neg(gain, "gain")?;

    match method {
        Tone::Rgb => {
            let mut lookup_table: [u8; 256] = [0; 256];
            util::generate_lookup_table(&mut lookup_table, |i| {
                (i as f64 * gain).round().clamp(0.0, 255.0) as u8
            });

            Ok(input.map_channels_if_alpha(|channel| lookup_table[channel as usize], |a| a))
        },
        Tone::Lab => {
            let mut lab = colorspace::srgb_to_lab(input, &White::D50);
            lab.edit_channel(|num| num * gain, 0);
            Ok(colorspace::lab_to_srgb(&lab, &White::D50))
        },
    }
}

/// Adjusts saturation by adding `saturation` to the saturation value (S) of `input` in HSV
///
/// # Arguments
///
/// * `saturation` - Must be between 0 and 255 (inclusive)
pub fn saturation(input: &Image<u8>, saturation: i32) -> ImgProcResult<Image<u8>> {
    error::check_in_range(saturation, 0, 255, "saturation")?;

    let mut hsv = colorspace::rgb_to_hsv(input);
    hsv.edit_channel(|s| (s + (saturation as f64 / 255.0)) as f64, 1);

    Ok(colorspace::hsv_to_rgb(&hsv))
}

/// Performs a gamma correction. `max` indicates the maximum allowed pixel value of the image
///
/// # Arguments
///
/// * `gamma` - Must be non-negative
pub fn gamma(input: &Image<u8>, gamma: f64, max: u8) -> ImgProcResult<Image<u8>> {
    error::check_non_neg(gamma, "gamma")?;

    Ok(input.map_channels_if_alpha(|channel| {
        ((channel as f64 / max as f64).powf(gamma) * (max as f64)).round() as u8
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
pub fn histogram_equalization(input: &Image<u8>, alpha: f64, ref_white: &White, precision: f64) -> ImgProcResult<Image<u8>> {
    error::check_non_neg(precision, "precision")?;
    error::check_in_range(alpha, 0.0, 1.0, "alpha")?;

    let mut lab = colorspace::srgb_to_lab(input, ref_white);
    let mut percentiles = HashMap::new();
    util::generate_histogram_percentiles(&lab, &mut percentiles, precision);

    lab.edit_channel(|num| {
        let key = (num * precision).round() as i32;
        (alpha * percentiles.get(&key).unwrap() * 100.0) + ((1.0 - alpha) * num)
    }, 0);

    Ok(colorspace::lab_to_srgb(&lab, ref_white))
}
