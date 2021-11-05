//! A module for SIMD operations

#![cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]

use crate::error;
use crate::image::{BaseImage, Image};
use crate::error::ImgProcResult;

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
use std::arch::x86_64::*;

#[cfg(all(feature = "simd", target_arch = "x86"))]
use std::arch::x86::*;

#[target_feature(enable = "avx2")]
pub unsafe fn adds_256(input: &Image<u8>, val: i16) -> ImgProcResult<Image<u8>> {
    let num_bytes = input.info().full_size();
    let mut data: Vec<u8> = vec![0; num_bytes as usize];
    let data_ptr: *mut u8 = data.as_mut_ptr();
    let input_ptr = input.data().as_ptr();

    let bias_256 = _mm256_set1_epi8(val.abs() as i8);

    let mut i = 0;
    while i < num_bytes {
        let chunk = _mm256_loadu_si256(input_ptr.add(i as usize)
            as *const __m256i);

        let res = if val > 0 {
            _mm256_adds_epu8(chunk, bias_256)
        } else {
            _mm256_subs_epu8(chunk, bias_256)
        };

        _mm256_storeu_si256(data_ptr.add(i as usize) as *mut __m256i, res);

        i += 32;
    }

    if i != 0 {
        for j in i..num_bytes {
            data.push((input.data()[j as usize] as i16 + val).clamp(0, 255) as u8);
        }
    }

    Ok(Image::from_vec(input.info().width, input.info().height, input.info().channels,
                    input.info().alpha, data))
}

#[target_feature(enable = "avx2")]
pub unsafe fn mask_adds_256(input: &Image<u8>, val: i16) -> ImgProcResult<Image<u8>> {
    error::check_alpha_channel(input.info().channels)?;

    let num_bytes = input.info().full_size();
    let mut data: Vec<u8> = vec![0; num_bytes as usize];
    let data_ptr: *mut u8 = data.as_mut_ptr();
    let input_ptr = input.data().as_ptr();

    let bias_256 = _mm256_set1_epi8(val.abs() as i8);
    let writemask = 0b11101110111011101110111011101110;

    let mut i = 0;
    while i < num_bytes {
        let chunk = _mm256_loadu_si256(input_ptr.add(i as usize)
            as *const __m256i);

        let res = if val > 0 {
            _mm256_mask_adds_epu8(chunk, writemask, chunk, bias_256)
        } else {
            _mm256_mask_subs_epu8(chunk, writemask, chunk, bias_256)
        };

        _mm256_storeu_si256(data_ptr.add(i as usize) as *mut __m256i, res);

        i += 32;
    }

    // if i != 0 {
    //     for j in i..num_bytes {
    //         if i % 3 != 0 {
    //             data.push((input.data()[j as usize] as i16 + val).clamp(0, 255) as u8);
    //         }
    //     }
    // }

    Ok(Image::from_vec(input.info().width, input.info().height, input.info().channels,
                    input.info().alpha, data))
}

pub fn check_mask_adds_256(input: &Image<u8>, val: i16) -> ImgProcResult<Image<u8>> {
    if input.info().channels == 3 {
        unsafe { adds_256(input, val) }
    } else {
        unsafe { mask_adds_256(input, val) }
    }
}