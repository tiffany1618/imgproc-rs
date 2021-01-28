use crate::util;
use crate::core::colorspace;
use crate::image::Image;
use crate::error::{ImgProcError, ImgProcResult};

use std::collections::HashMap;

/// Adjusts brightness by adding `bias` to each RGB channel
pub fn brightness_rgb(input: &Image<u8>, bias: i32) -> ImgProcResult<Image<u8>> {
    if bias < 0 || bias > 255 {
        return Err(ImgProcError::InvalidArgument("bias is not in range 0 to 255".to_string()));
    }

    let mut lookup_table: [u8; 256] = [0; 256];
    util::create_lookup_table(&mut lookup_table, |i| {
        (i as i32 + bias) as u8
    });

    Ok(input.map_channels_if_alpha(|channel| lookup_table[channel as usize], |a| a))
}

/// Adjusts brightness by adding `bias` to the luminance value (Y) of `input` in CIE XYZ
pub fn brightness_xyz(input: &Image<u8>, bias: i32) -> ImgProcResult<Image<u8>> {
    if bias < 0 || bias > 255 {
        return Err(ImgProcError::InvalidArgument("bias is not in range 0 to 255".to_string()));
    }

    let mut xyz = colorspace::srgb_to_xyz(input);
    xyz.edit_channel(|num| num + (bias as f64 / 255.0), 1);
    Ok(colorspace::xyz_to_srgb(&xyz))
}

/// Adjusts contrast by multiplying each RGB channel by `gain`
// gain > 0
pub fn contrast_rgb(input: &Image<u8>, gain: f64) -> ImgProcResult<Image<u8>> {
    if gain <= 0.0 {
        return Err(ImgProcError::InvalidArgument("gain is negative".to_string()));
    }

    let mut lookup_table: [u8; 256] = [0; 256];
    util::create_lookup_table(&mut lookup_table, |i| {
        (i as f64 * gain).round() as u8
    });

    Ok(input.map_channels_if_alpha(|channel| lookup_table[channel as usize], |a| a))
}

/// Adjusts contrast by multiplying luminance value (Y) of `input` in CIE XYZ by `gain`
// gain > 0
pub fn contrast_xyz(input: &Image<u8>, gain: f64) -> ImgProcResult<Image<u8>> {
    if gain <= 0.0 {
        return Err(ImgProcError::InvalidArgument("gain is negative".to_string()));
    }

    let mut xyz = colorspace::srgb_to_xyz(input);
    xyz.edit_channel(|num| num * gain, 1);
    Ok(colorspace::xyz_to_srgb(&xyz))
}

/// Performs a histogram equalization on `input`
///
/// # Arguments
///
/// * `alpha` - Represents the amount of equalization, where 0 corresponds to no equalization and
/// 1 corresponds to full equalization
/// * `ref_white` - A string slice representing the reference white value of the image
/// * `precision` - See the function `util::generate_histogram_percentiles`
pub fn histogram_equalization(input: &Image<u8>, alpha: f64, ref_white: &str, precision: f64) -> ImgProcResult<Image<u8>> {
    if alpha < 0.0 || alpha > 1.0 {
        return Err(ImgProcError::InvalidArgument("alpha is not in range 0 to 1".to_string()));
    } else if precision <= 0.0 {
        return Err(ImgProcError::InvalidArgument("precision is not positive".to_string()));
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

// TODO: Error handling here
impl Image<f64> {
    /// Adjusts brightness by adding `bias` to each RGB channel
    pub fn brightness_rgb(&mut self, bias: i32) {
        let mut lookup_table: [u8; 256] = [0; 256];
        util::create_lookup_table(&mut lookup_table, |i| {
            (i as i32 + bias) as u8
        });

        self.apply_channels_if_alpha(|channel| lookup_table[channel as usize] as f64, |a| a)
    }

    /// Adjusts brightness by adding `bias` to the luminance value (Y) of `input` in CIE XYZ
    pub fn brightness_xyz(&mut self, bias: i32) {
        self.srgb_to_xyz();
        self.edit_channel(|num| num + (bias as f64 / 255.0), 1);
        self.xyz_to_srgb();
    }

    /// Adjusts contrast by multiplying each RGB channel by `gain`
    // gain > 0
    pub fn contrast_rgb(&mut self, gain: f64) {
        if gain <= 0.0 {
            return;
        }

        let mut lookup_table: [u8; 256] = [0; 256];
        util::create_lookup_table(&mut lookup_table, |i| {
            (i as f64 * gain).round() as u8
        });

        self.apply_channels_if_alpha(|channel| lookup_table[channel as usize] as f64, |a| a)
    }

    /// Adjusts contrast by multiplying luminance value (Y) of `input` in CIE XYZ by `gain`
    // gain > 0
    pub fn contrast_xyz(&mut self, gain: f64) {
        if gain <= 0.0 {
            return;
        }

        self.srgb_to_xyz();
        self.edit_channel(|num| num * gain, 1);
        self.xyz_to_srgb();
    }

    /// Performs a histogram equalization on `input`
    ///
    /// # Arguments
    ///
    /// * `alpha` - Represents the amount of equalization, where 0 corresponds to no equalization and
    /// 1 corresponds to full equalization
    /// * `ref_white` - A string slice representing the reference white value of the image
    /// * `precision` - See the function `util::generate_histogram_percentiles`
    pub fn histogram_equalization(&mut self, alpha: f64, ref_white: &str, precision: f64) {
        if alpha < 0.0 || alpha > 1.0 || precision <= 0.0 {
            return;
        }

        self.srgb_to_lab(ref_white);
        let mut percentiles = HashMap::new();
        util::generate_histogram_percentiles(self, &mut percentiles, precision);

        self.edit_channel(|num| {
            let key = (num * precision).round() as i32;
            (alpha * percentiles.get(&key).unwrap() * 100.0) + ((1.0 - alpha) * num)
        }, 0);

        self.lab_to_srgb(ref_white)
    }
}
