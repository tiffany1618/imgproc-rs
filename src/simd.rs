//! A module for SIMD operations

use crate::image::{BaseImage, Image};
use crate::error::{ImgProcError, ImgProcResult};

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
use std::arch::x86_64::*;

#[cfg(all(feature = "simd", target_arch = "x86"))]
use std::arch::x86::*;

/// Adds `val` to each 8-bit channel of `input` using saturation, ignoring the alpha channel
/// if present
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[target_feature(enable = "avx2")]
pub unsafe fn adds_256_u8(input: &Image<u8>, val: i16) -> Image<u8> {
    let num_bytes = input.info().full_size();
    let mut data: Vec<u8> = vec![0; num_bytes as usize];

    // Load val into 256-bit SIMD register
    let mut val_256 = _mm256_set1_epi8(val.abs() as i8);

    // Mask the alpha bit if present
    if input.info().alpha {
        let zeroes_256 = _mm256_setzero_si256();
        let alpha_mask = _mm256_set1_epi32(-2 * 2_i32.pow(30));
        val_256 = _mm256_blendv_epi8(val_256, zeroes_256, alpha_mask);
    }

    // Process 32 bytes at a time
    let mut i = 0;
    while (i + 32) <= num_bytes {
        // Load 32-byte chunk into 256-bit SIMD register
        let chunk = _mm256_loadu_si256(input.data().as_ptr().
            offset(i as isize) as *const _);

        // Add or subtract val from each 8-bit integer with saturation
        let res = if val > 0 {
            _mm256_adds_epu8(chunk, val_256)
        } else {
            _mm256_subs_epu8(chunk, val_256)
        };

        // Store 32 bytes in output vector
        _mm256_storeu_si256(data.as_mut_ptr().offset(i as isize) as *mut _, res);

        i += 32;
    }

    // Process the remaining bytes normally
    if i > num_bytes {
        for j in (i - 32)..num_bytes {
            data[j as usize] = (input.data()[j as usize] as i16 + val).clamp(0, 255) as u8;
        }
    }

    Image::from_vec(input.info().width, input.info().height, input.info().channels,
                    input.info().alpha, data)
}

/// Adds `val` to every `n`th 8-bit channel of `input` using saturation.
/// If `n` is an invalid channel number, adds `val` to all channels.
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[target_feature(enable = "avx2")]
pub unsafe fn adds_n_256_u8(input: &Image<u8>, val: i16, n: u8) -> Image<u8> {
    let num_bytes = input.info().full_size();
    let mut data: Vec<u8> = vec![0; num_bytes as usize];

    let mut val_256 = _mm256_set1_epi8(val.abs() as i8);
    let zeroes_256 = _mm256_setzero_si256();

    // Create masks for the channels we want to ignore
    let mask = if input.info().alpha {
        match n {
            0 => _mm256_set1_epi32(0x80),
            1 => _mm256_set1_epi32(0x8000),
            2 => _mm256_set1_epi32(0x800000),
            3 => _mm256_set1_epi32(-0x80000000),
            _ => _mm256_setzero_si256()
        }
    } else {
        match n {
            0 => _mm256_set_epi64x(0x80000080, 0x800000800000,
                                   -0x7FFFFF7FFFFF8000, 0x80000080000080),
            1 => _mm256_set_epi64x(0x8000008000, 0x80000080000080,
                                   0x800000800000, -0x7FFFFF7FFFFF8000),
            2 => _mm256_set_epi64x(0x800000800000, -0x7FFFFF7FFFFF8000,
                                   0x80000080000080, 0x800000800000),
            _ => _mm256_setzero_si256()
        }
    };
    val_256 = _mm256_blendv_epi8(zeroes_256, val_256, mask);

    // In order to reuse the mask for every chunk in a 3-channel pixel, we can only process chunks
    // that are multiples of 8 * 3 = 24 bits. The largest such chunk that fits in 256 bits is
    // 24 * 10 = 240 bits = 30 bytes.
    let mut step = 30;

    // 4-channel pixels fit evenly within 32-byte chunks
    if input.info().alpha {
        step = 32;
    }

    let mut i = 0;
    while (i + step) <= num_bytes {
        let chunk = _mm256_loadu_si256(input.data().as_ptr().
            offset(i as isize) as *const _);

        let res = if val > 0 {
            _mm256_adds_epu8(chunk, val_256)
        } else {
            _mm256_subs_epu8(chunk, val_256)
        };

        _mm256_storeu_si256(data.as_mut_ptr().offset(i as isize) as *mut _, res);

        i += step;
    }

    if i > num_bytes {
        for j in ((i - step)..num_bytes).step_by(input.info().channels as usize) {
            let index = j as usize + n as usize;
            data[index] = (input.data()[index] as i16 + val).clamp(0, 255) as u8;
        }
    }

    Image::from_vec(input.info().width, input.info().height, input.info().channels,
                       input.info().alpha, data)
}

/// Separates a 3-channel input image into 3 256-bit wide integer vectors, starting at the channel
/// denoted by `offset`. Does not check if `offset` is valid.
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[target_feature(enable = "avx2")]
pub unsafe fn deinterleave_3_256_u8(input: &Image<u8>, offset: usize) -> (__m256i, __m256i, __m256i) {
    let step = input.info().channels as usize;
    let range = 32 * step;
    let mut r = Vec::with_capacity(256);
    let mut g = Vec::with_capacity(256);
    let mut b = Vec::with_capacity(256);

    // Separate channels into different vectors
    for i in (offset..(offset + range)).step_by(step) {
        r.push(input.data()[i]);
        g.push(input.data()[i + 1]);
        b.push(input.data()[i + 2]);
    }

    // Load each vector into a 256-bit SIMD register
    let r_256 = _mm256_loadu_si256(r.as_mut_ptr() as *mut _);
    let g_256 = _mm256_loadu_si256(g.as_mut_ptr() as *mut _);
    let b_256 = _mm256_loadu_si256(b.as_mut_ptr() as *mut _);

    (r_256, g_256, b_256)
}

/// Separates a 4-channel input image into 4 256-bit wide integer vectors, starting at the channel
/// denoted by `offset`. Does not check if `offset` is valid.
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

/// Computes the average of each 3-channel pixel of `input` and returns a grayscale image
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

        // Approximate average
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

/// Computes the average of each 4-channel pixel of `input`, ignoring the alpha channel, and
/// returns a grayscale image
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

        // Approximate average
        let avg_rg = _mm256_avg_epu8(r, g);
        let avg_rgb = _mm256_avg_epu8(avg_rg, b);

        // Interleave the alpha and grayscale channels
        let unpacked_lo = _mm256_unpacklo_epi8(avg_rgb, a);
        let unpacked_hi = _mm256_unpackhi_epi8(avg_rgb, a);

        // Store the interleaved channels in the output vector
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

/// Computes the average of each of `input`, ignoring the alpha channel if present, and
/// returns a grayscale image
#[cfg(feature = "simd")]
pub fn avg_checked_256_u8(input: &Image<u8>) -> Image<u8> {
    if input.info().alpha {
        unsafe { avg_alpha_256_u8(input) }
    } else {
        unsafe { avg_256_u8(input) }
    }
}