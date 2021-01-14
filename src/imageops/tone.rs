use crate::util;
use crate::imageops::colorspace;
use crate::image::Image;

use std::collections::HashMap;

// Adjust brightness by adding the bias to each RGB channel
pub fn brightness_rgb(input: &Image<u8>, bias: i32) -> Image<u8> {
    let mut lookup_table: [u8; 256] = [0; 256];
    util::create_lookup_table(&mut lookup_table, |i| {
        (i as i32 + bias) as u8
    });

    input.map_channels_if_alpha(|channel| lookup_table[channel as usize], |a| a)
}

// Adjust brightness by adding the bias to the luminance value (Y)
pub fn brightness_xyz(input: &Image<u8>, bias: i32) -> Image<u8> {
    let mut xyz = colorspace::srgb_to_xyz(input);
    xyz.edit_channel(|num| num + (bias as f64 / 255.0), 1);
    colorspace::xyz_to_srgb(&xyz)
}

// Adjust contrast by multiplying each RGB channel by gain
// gain > 0
pub fn contrast_rgb(input: &Image<u8>, gain: f64) -> Option<Image<u8>> {
    if gain <= 0.0 {
        return None;
    }

    let mut lookup_table: [u8; 256] = [0; 256];
    util::create_lookup_table(&mut lookup_table, |i| {
        (i as f64 * gain).round() as u8
    });

    Some(input.map_channels_if_alpha(|channel| lookup_table[channel as usize], |a| a))
}

// Adjust contrast by multiplying luminance value (Y) by gain
// gain > 0
pub fn contrast_xyz(input: &Image<u8>, gain: f64) -> Option<Image<u8>> {
    if gain <= 0.0 {
        return None;
    }

    let mut xyz = colorspace::srgb_to_xyz(input);
    xyz.edit_channel(|num| num * gain, 1);
    Some(colorspace::xyz_to_srgb(&xyz))
}

// alpha range [0, 1];
// 0 corresponds to no equalization,
// 1 corresponds to full equalization
pub fn histogram_equalization(input: &Image<u8>, alpha: f64, ref_white: &str, precision: f64) -> Option<Image<u8>> {
    if alpha < 0.0 || alpha > 1.0 || precision <= 0.0 {
        return None;
    }

    let mut lab = colorspace::srgb_to_lab(input, ref_white);
    let mut percentiles = HashMap::new();
    util::generate_histogram_percentiles(&lab, &mut percentiles, precision);

    lab.edit_channel(|num| {
        let key = (num * precision).round() as i32;
        (alpha * percentiles.get(&key).unwrap() * 100.0) + ((1.0 - alpha) * num)
    }, 0);

    Some(colorspace::lab_to_srgb(&lab, ref_white))
}
