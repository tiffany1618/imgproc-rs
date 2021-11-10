//! A module for SIMD operations

use crate::error;
use crate::image::{BaseImage, Image};
use crate::error::ImgProcResult;

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
use std::arch::x86_64::*;

#[cfg(all(feature = "simd", target_arch = "x86"))]
use std::arch::x86::*;

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[target_feature(enable = "avx2")]
pub unsafe fn adds_256_u8(input: &Image<u8>, val: i16) -> ImgProcResult<Image<u8>> {
    let num_bytes = input.info().full_size();
    let mut data: Vec<u8> = vec![0; num_bytes as usize];

    let val_256 = _mm256_set1_epi8(val.abs() as i8);

    let mut i = 0;
    while (i + 32) <= num_bytes {
        let chunk = _mm256_loadu_si256(input.data().as_ptr().
            offset(i as isize) as *const _);

        let res = if val > 0 {
            _mm256_adds_epu8(chunk, val_256)
        } else {
            _mm256_subs_epu8(chunk, val_256)
        };

        _mm256_storeu_si256(data.as_mut_ptr().offset(i as isize) as *mut _, res);

        i += 32;
    }

    if i > num_bytes {
        for j in (i - 32)..num_bytes {
            data[j as usize] = (input.data()[j as usize] as i16 + val).clamp(0, 255) as u8;
        }
    }

    Ok(Image::from_vec(input.info().width, input.info().height, input.info().channels,
                    input.info().alpha, data))
}

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[target_feature(enable = "avx2")]
pub unsafe fn masked_adds_256_u8(input: &Image<u8>, val: i16) -> ImgProcResult<Image<u8>> {
    error::check_alpha(input.info().alpha)?;

    let num_bytes = input.info().full_size();
    let mut data: Vec<u8> = vec![0; num_bytes as usize];

    let val_256 = _mm256_set1_epi8(val.abs() as i8);
    let alpha_mask = _mm256_set1_epi32(-2 * 2_i32.pow(30));

    let mut i = 0;
    while (i + 32) <= num_bytes {
        let chunk = _mm256_loadu_si256(input.data().as_ptr().
            offset(i as isize) as *const _);

        let res = if val > 0 {
            _mm256_adds_epu8(chunk, val_256)
        } else {
            _mm256_subs_epu8(chunk, val_256)
        };

        let masked_res = _mm256_blendv_epi8(res, chunk, alpha_mask);
        _mm256_storeu_si256(data.as_mut_ptr().offset(i as isize) as *mut _, masked_res);

        i += 32;
    }

    Ok(Image::from_vec(input.info().width, input.info().height, input.info().channels,
                    input.info().alpha, data))
}

pub fn check_adds_256_u8(input: &Image<u8>, val: i16) -> ImgProcResult<Image<u8>> {
    if input.info().alpha {
        unsafe { masked_adds_256_u8(input, val) }
    } else {
        unsafe { adds_256_u8(input, val) }
    }
}