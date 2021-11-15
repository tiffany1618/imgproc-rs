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
pub unsafe fn adds_masked_256_u8(input: &Image<u8>, val: i16) -> ImgProcResult<Image<u8>> {
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

#[cfg(feature = "simd")]
pub fn adds_checked_256_u8(input: &Image<u8>, val: i16) -> ImgProcResult<Image<u8>> {
    if input.info().alpha {
        unsafe { adds_masked_256_u8(input, val) }
    } else {
        unsafe { adds_256_u8(input, val) }
    }
}

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[target_feature(enable = "avx2")]
pub unsafe fn deinterleave_3_256_u8(input: &Image<u8>, offset: usize) -> (__m256i, __m256i, __m256i) {
    let step = input.info().channels as usize;
    let range = 32 * step;
    let mut r = Vec::with_capacity(256);
    let mut g = Vec::with_capacity(256);
    let mut b = Vec::with_capacity(256);

    for i in (offset..(offset + range)).step_by(step) {
        r.push(input.data()[i]);
        g.push(input.data()[i + 1]);
        b.push(input.data()[i + 2]);
    }

    let r_256 = _mm256_loadu_si256(r.as_mut_ptr() as *mut _);
    let g_256 = _mm256_loadu_si256(g.as_mut_ptr() as *mut _);
    let b_256 = _mm256_loadu_si256(b.as_mut_ptr() as *mut _);

    (r_256, g_256, b_256)
}

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[target_feature(enable = "avx2")]
pub unsafe fn deinterleave_4_256_u8(input: &Image<u8>, offset: usize) -> (__m256i, __m256i, __m256i, __m256i) {
    let step = input.info().channels as usize;
    let range = 32 * step;
    let mut r = Vec::with_capacity(256);
    let mut g = Vec::with_capacity(256);
    let mut b = Vec::with_capacity(256);
    let mut a = Vec::with_capacity(256);

    for i in (offset..(offset + range)).step_by(step) {
        r.push(input.data()[i]);
        g.push(input.data()[i + 1]);
        b.push(input.data()[i + 2]);
        a.push(input.data()[i + 3]);
    }

    let r_256 = _mm256_loadu_si256(r.as_mut_ptr() as *mut _);
    let g_256 = _mm256_loadu_si256(g.as_mut_ptr() as *mut _);
    let b_256 = _mm256_loadu_si256(b.as_mut_ptr() as *mut _);
    let a_256 = _mm256_loadu_si256(a.as_mut_ptr() as *mut _);

    (r_256, g_256, b_256, a_256)
}

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[target_feature(enable = "avx2")]
pub unsafe fn avg_256_u8(input: &Image<u8>) -> Image<u8> {
    let channels = input.info().channels as usize;
    let chunk_size = 32 * channels;
    let num_bytes = input.info().full_size() as usize;
    let mut data: Vec<u8> = vec![0; (num_bytes / 3) as usize];

    let mut i: usize= 0;
    while (i + chunk_size) <= num_bytes {
        let (r, g, b) = deinterleave_3_256_u8(input, i);
        let avg_rg = _mm256_avg_epu8(r, g);
        let avg_rgb = _mm256_avg_epu8(avg_rg, b);
        _mm256_storeu_si256(data.as_mut_ptr().offset((i / channels) as isize)
                                as *mut _, avg_rgb);

        i += chunk_size;
    }

    if i > num_bytes {
        for j in ((i - chunk_size)..num_bytes).step_by(3) {
            let sum = (input.data()[i] + input.data()[i + 1] + input.data()[i + 2]) as f32;
            data[j / channels] = (sum / 3.0).round() as u8;
        }
    }

    Image::from_vec(input.info().width, input.info().height, 1,
                       input.info().alpha, data)
}

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[target_feature(enable = "avx2")]
pub unsafe fn avg_alpha_256_u8(input: &Image<u8>) -> Image<u8> {
    let channels = input.info().channels as usize;
    let chunk_size = 32 * channels;
    let num_bytes = input.info().full_size() as usize;
    let mut data: Vec<u8> = vec![0; (num_bytes / 2) as usize];

    let mut i: usize= 0;
    while (i + chunk_size) <= num_bytes {
        let (r, g, b, a) = deinterleave_4_256_u8(input, i);
        let avg_rg = _mm256_avg_epu8(r, g);
        let avg_rgb = _mm256_avg_epu8(avg_rg, b);

        let unpacked_lo = _mm256_unpacklo_epi8(avg_rgb, a);
        let unpacked_hi = _mm256_unpackhi_epi8(avg_rgb, a);
        _mm256_storeu2_m128i(data.as_mut_ptr().offset((i / 2) as isize + 32) as *mut _,
                            data.as_mut_ptr().offset((i / 2) as isize) as *mut _,
                            unpacked_lo);
        _mm256_storeu2_m128i(data.as_mut_ptr().offset((i / 2) as isize + 48) as *mut _,
                             data.as_mut_ptr().offset((i / 2) as isize + 16) as *mut _,
                             unpacked_hi);

        i += chunk_size;
    }

    if i > num_bytes {
        for j in ((i - chunk_size)..num_bytes).step_by(channels) {
            let sum = (input.data()[i] + input.data()[i + 1] + input.data()[i + 2]) as f32;
            data[j / 2] = (sum / 3.0).round() as u8;
            data[j / 2 + 1] = input.data()[i + 3];
        }
    }

    Image::from_vec(input.info().width, input.info().height, 2,
                    input.info().alpha, data)
}

#[cfg(feature = "simd")]
pub fn avg_checked_256_u8(input: &Image<u8>) -> Image<u8> {
    if input.info().alpha {
        unsafe { avg_alpha_256_u8(input) }
    } else {
        unsafe { avg_256_u8(input) }
    }
}