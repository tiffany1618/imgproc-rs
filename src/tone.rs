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
pub fn brightness(input: &Image<u8>, bias: i32, method: Tone) -> ImgProcResult<Image<u8>> {
    error::check_in_range(bias, 0, 255, "bias")?;

    match method {
        Tone::Rgb => {
            if is_x86_feature_detected!("avx2") {
                unsafe {
                    Ok(brightness_rgb_avx(input, bias))
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

pub fn brightness_norm(input: &Image<u8>, bias: i32) -> Image<u8> {
    brightness_rgb(input, bias)
}

pub fn brightness_avx2(input: &Image<u8>, bias: i32) -> Image<u8> {
    unsafe { brightness_rgb_avx(input, bias) }
}

unsafe fn brightness_rgb_avx(input: &Image<u8>, bias: i32) -> Image<u8> {
    let mut add = true;
    if bias < 0 {
        add = false;
    }
    let bias_i8 = bias.abs() as i8;
    let bias_m256i = _mm256_set_epi8(bias_i8, bias_i8, bias_i8, bias_i8,
                                     bias_i8, bias_i8, bias_i8, bias_i8,
                                     bias_i8, bias_i8, bias_i8, bias_i8,
                                     bias_i8, bias_i8, bias_i8, bias_i8,
                                     bias_i8, bias_i8, bias_i8, bias_i8,
                                     bias_i8, bias_i8, bias_i8, bias_i8,
                                     bias_i8, bias_i8, bias_i8, bias_i8,
                                     bias_i8, bias_i8, bias_i8, bias_i8);

    let output = input.data().chunks(32)
        .map(|vec| match vec {
            &[v0, v1, v2, v3, v4, v5, v6, v7,
                v8, v9, v10, v11, v12, v13, v14, v15,
                v16, v17, v18, v19, v20, v21, v22, v23,
                v24, v25, v26, v27, v28, v29, v30, v31] => {
                let chunk_m256i = _mm256_set_epi8(v0 as i8, v1 as i8, v2 as i8, v3 as i8, v4 as i8, v5 as i8, v6 as i8, v7 as i8,
                                                  v8 as i8, v9 as i8, v10 as i8, v11 as i8, v12 as i8, v13 as i8, v14 as i8, v15 as i8,
                                                  v16 as i8, v17 as i8, v18 as i8, v19 as i8, v20 as i8, v21 as i8, v22 as i8, v23 as i8,
                                                  v24 as i8, v25 as i8, v26 as i8, v27 as i8, v28 as i8, v29 as i8, v30 as i8, v31 as i8);

                if add {
                    let res = _mm256_adds_epu8(chunk_m256i, bias_m256i);
                    let res_unpacked: (u8, u8, u8, u8, u8, u8, u8, u8,
                                       u8, u8, u8, u8, u8, u8, u8, u8,
                                       u8, u8, u8, u8, u8, u8, u8, u8,
                                       u8, u8, u8, u8, u8, u8, u8, u8) = mem::transmute(res);
                    return vec![res_unpacked.31, res_unpacked.30, res_unpacked.29, res_unpacked.28,
                                res_unpacked.27, res_unpacked.26, res_unpacked.25, res_unpacked.24,
                                res_unpacked.23, res_unpacked.22, res_unpacked.21, res_unpacked.20,
                                res_unpacked.19, res_unpacked.18, res_unpacked.17, res_unpacked.16,
                                res_unpacked.15, res_unpacked.14, res_unpacked.13, res_unpacked.12,
                                res_unpacked.11, res_unpacked.10, res_unpacked.9, res_unpacked.8,
                                res_unpacked.7, res_unpacked.6, res_unpacked.5, res_unpacked.4,
                                res_unpacked.3, res_unpacked.2, res_unpacked.1, res_unpacked.0];
                } else {
                    let res = _mm256_subs_epu8(chunk_m256i, bias_m256i);
                    let res_unpacked: (u8, u8, u8, u8, u8, u8, u8, u8,
                                       u8, u8, u8, u8, u8, u8, u8, u8,
                                       u8, u8, u8, u8, u8, u8, u8, u8,
                                       u8, u8, u8, u8, u8, u8, u8, u8) = mem::transmute(res);
                    return vec![res_unpacked.31, res_unpacked.30, res_unpacked.29, res_unpacked.28,
                                res_unpacked.27, res_unpacked.26, res_unpacked.25, res_unpacked.24,
                                res_unpacked.23, res_unpacked.22, res_unpacked.21, res_unpacked.20,
                                res_unpacked.19, res_unpacked.18, res_unpacked.17, res_unpacked.16,
                                res_unpacked.15, res_unpacked.14, res_unpacked.13, res_unpacked.12,
                                res_unpacked.11, res_unpacked.10, res_unpacked.9, res_unpacked.8,
                                res_unpacked.7, res_unpacked.6, res_unpacked.5, res_unpacked.4,
                                res_unpacked.3, res_unpacked.2, res_unpacked.1, res_unpacked.0];
                }
            },
            _ => unimplemented!(),
        })
        .flatten()
        .collect();

    Image::from_vec(input.info().width, input.info().height, input.info().channels,
                    input.info().alpha, output)
}

fn brightness_rgb(input: &Image<u8>, bias: i32) -> Image<u8> {
    let mut lookup_table: [u8; 256] = [0; 256];
    util::generate_lookup_table(&mut lookup_table, |i| {
        (i as i32 + bias).clamp(0, 255) as u8
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
