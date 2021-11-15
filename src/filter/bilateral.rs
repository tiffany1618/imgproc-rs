#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::{colorspace, error, util};
use crate::enums::{Bilateral, White};
use crate::error::ImgProcResult;
use crate::image::{BaseImage, Image};

/// Applies a bilateral filter using CIE LAB
#[cfg(not(feature = "rayon"))]
pub fn bilateral_filter(input: &Image<u8>, range: f32, spatial: f32, algorithm: Bilateral)
    -> ImgProcResult<Image<u8>> {
    error::check_non_neg(range, "range")?;
    error::check_non_neg(spatial, "spatial")?;

    let (width, height, channels) = input.info().whc();
    let size = ((spatial * 4.0) + 1.0) as u32;
    let spatial_mat = util::generate_spatial_mat(size, spatial)?;

    let lab = colorspace::srgb_to_lab_f32(&input, &White::D65);
    let mut output = Image::blank(lab.info());
    let mut p_out = Vec::with_capacity(channels as usize);

    match algorithm {
        Bilateral::Direct => {
            for y in 0..height {
                for x in 0..width {
                    bilateral_direct_pixel(&lab, &mut p_out, range, &spatial_mat, size, x, y);
                    output.set_pixel(x, y, &p_out);
                }
            }
        },
    }

    Ok(colorspace::lab_to_srgb_f32(&output, &White::D65))
}

/// Applies a bilateral filter using CIE LAB
#[cfg(feature = "rayon")]
pub fn bilateral_filter(input: &Image<u8>, range: f32, spatial: f32, algorithm: Bilateral)
                            -> ImgProcResult<Image<u8>> {
    error::check_non_neg(range, "range")?;
    error::check_non_neg(spatial, "spatial")?;

    let (width, height, channels, alpha) = input.info().whca();
    let size = ((spatial * 4.0) + 1.0) as u32;
    let spatial_mat = util::generate_spatial_mat(size, spatial)?;

    let lab = colorspace::srgb_to_lab_f32(&input, &White::D65);

    match algorithm {
        Bilateral::Direct => {
            let data: Vec<Vec<f32>> = (0..input.info().size())
                .into_par_iter()
                .map(|i| {
                    let (x, y) = util::get_2d_coords(i, width);
                    bilateral_direct_pixel(&lab, range, &spatial_mat, size, x, y)
                })
                .collect();

            let output = Image::from_vec_of_vec(width, height, channels, alpha, data);
            Ok(colorspace::lab_to_srgb_f32(&output, &White::D65))
        },
    }
}

#[cfg(not(feature = "rayon"))]
fn bilateral_direct_pixel(input: &Image<f32>, output: &mut Vec<f32>, range: f32, spatial_mat: &[f32], size: u32, x: u32, y: u32) {
    let p_n = input.get_neighborhood_2d(x, y, size as u32);
    let p_in = input.get_pixel(x, y);
    output.clear();

    for (c, channel) in p_in.iter().enumerate() {
        let mut total_weight = 0.0;
        let mut p_curr = 0.0;

        for i in 0..((size * size) as usize) {
            let g_r = util::gaussian_fn((channel - p_n[i][c]).abs(), range).unwrap();
            let weight = spatial_mat[i] * g_r;

            p_curr += weight * p_n[i][c];
            total_weight += weight;
        }

        output.push(p_curr / total_weight);
    }
}

#[cfg(feature = "rayon")]
fn bilateral_direct_pixel(input: &Image<f32>, range: f32, spatial_mat: &[f32], size: u32, x: u32, y: u32) -> Vec<f32> {
    let p_n = input.get_neighborhood_2d(x, y, size as u32);
    let p_in = input.get_pixel(x, y);
    let mut p_out = Vec::with_capacity(input.info().channels as usize);

    for (c, channel) in p_in.iter().enumerate() {
        let mut total_weight = 0.0;
        let mut p_curr = 0.0;

        for i in 0..((size * size) as usize) {
            let g_r = util::gaussian_fn((channel - p_n[i][c]).abs(), range).unwrap();
            let weight = spatial_mat[i] * g_r;

            p_curr += weight * p_n[i][c];
            total_weight += weight;
        }

        p_out.push(p_curr / total_weight);
    }

    p_out
}