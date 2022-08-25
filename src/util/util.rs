use std::collections::{BTreeMap, HashMap};
use std::f32::consts::{E, PI};

use crate::enums::White;
use crate::error;
use crate::error::ImgProcResult;
use crate::image::{BaseImage, Image, Number};
use crate::util::math;

////////////////////////////
// Image helper functions
////////////////////////////

/// Returns a tuple representing the XYZ tristimulus values for a given reference white value
pub fn xyz_tristimulus_vals(ref_white: &White) -> (f32, f32, f32) {
    match ref_white {
        White::D50 => (96.4212, 100.0, 82.5188),
        White::D65 => (95.0489, 100.0, 108.8840),
    }
}

/// A helper function for histogram equalization
///
/// # Arguments
///
/// * `input` - a reference to a CIELAB `Image`
/// * `percentiles` - a mutable `HashMap` reference relating an L channel intensity to the number
/// of times it occurs in `input` as a percentile
/// * `precision` - The range of possible L channel intensity values (used to convert the intensity
/// value to an i32, which can be used as a key in `HashMap` and `BTreeMap`)
pub fn generate_histogram_percentiles(input: &Image<f32>, percentiles: &mut HashMap<i32, f32>, precision: f32) {
    let mut histogram = BTreeMap::new();

    for y in 0..(input.info().height) {
        for x in 0..(input.info().width) {
            let p = (input.get_pixel(x, y)[0] * precision).round() as i32;
            let count = histogram.entry(p).or_insert(1);
            *count += 1;
        }
    }

    let mut sum: i32 = 0;
    let num_pixels = input.info().size() as f32;
    for (key, val) in &histogram {
        sum += val;
        percentiles.insert(*key, sum as f32 / num_pixels);
    }
}

/// Populates `table` with the appropriate values based on function `f`
pub fn generate_lookup_table<T: Number, F>(table: &mut [T; 256], f: F)
    where F: Fn(u8) -> T {
    for (i, num) in table.iter_mut().enumerate() {
        *num = f(i as u8);
    }
}

/// Generates a Gaussian kernel
pub fn generate_gaussian_kernel(size: u32, sigma: f32) -> ImgProcResult<Vec<f32>> {
    error::check_odd(size, "size")?;

    let mut filter = vec![0.0; (size * size) as usize];
    let k = ((size - 1) / 2) as i32;

    for i in 0..(size as i32) {
        for j in 0..(size as i32) {
            if i <= j {
                let num = (1.0 / (2.0 * PI * sigma * sigma)) *
                    (E.powf(-(((i - k) * (i - k) + (j - k) * (j - k)) as f32) / (2.0 * sigma * sigma)));
                filter[(i * size as i32 + j) as usize] = num;

                if i != j {
                    filter[(j * size as i32 + i) as usize] = num;
                }
            }
        }
    }

    Ok(filter)
}

/// Generates a Laplacian of Gaussian kernel
pub fn generate_log_kernel(size: u32, sigma: f32) -> ImgProcResult<Vec<f32>> {
    error::check_odd(size, "size")?;

    let mut filter = vec![0.0; (size * size) as usize];
    let k = ((size - 1) / 2) as i32;

    for i in 0..(size as i32) {
        for j in 0..(size as i32) {
            if i <= j {
                let exp = -(((i - k) * (i - k) + (j - k) * (j - k)) as f32) / (2.0 * sigma * sigma);
                let num = (-1.0 / (PI * sigma.powf(4.0))) * (1.0 - exp) * (E.powf(exp));
                filter[(i * size as i32 + j) as usize] = num;

                if i != j {
                    filter[(j * size as i32 + i) as usize] = num;
                }
            }
        }
    }

    Ok(filter)
}

/// Generates a matrix of distances relative to the center of the matrix
pub fn generate_spatial_mat(size: u32, spatial: f32) -> ImgProcResult<Vec<f32>> {
    let center = size / 2;
    let mut mat = vec![0.0; (size * size) as usize];

    for y in 0..(center + 1) {
        for x in 0..(center + 1) {
            if mat[(y * size + x) as usize] == 0.0 && !(x == center && y == center) {
                let dist = math::distance(center, center, x, y);
                let g = math::gaussian_fn(dist, spatial)?;
                mat[(y * size + x) as usize] = g;

                if x == y {
                    let delta = center - y;
                    let coord = center + delta;

                    mat[(coord * size + x) as usize] = g;
                    mat[(x * size + coord) as usize] = g;
                    mat[(coord * size + coord) as usize] = g;
                } else if x == center {
                    let delta = center - y;

                    mat[(x * size + x - delta) as usize] = g;
                    mat[(x * size + x + delta) as usize] = g;
                    mat[((x + delta) * size + x) as usize] = g;
                } else {
                    let delta_x = center - x;
                    let delta_y = center - y;
                    let pos_x = center + delta_x;
                    let pos_y = center + delta_y;
                    let neg_x = center - delta_x;
                    let neg_y = center - delta_y;

                    mat[(neg_x * size + neg_y) as usize] = g;
                    mat[(neg_y * size + pos_x) as usize] = g;
                    mat[(pos_x * size + neg_y) as usize] = g;
                    mat[(pos_y * size + neg_x) as usize] = g;
                    mat[(neg_x * size + pos_y) as usize] = g;
                    mat[(pos_y * size + pos_x) as usize] = g;
                    mat[(pos_x * size + pos_y) as usize] = g;
                }
            }
        }
    }

    Ok(mat)
}

/// Generates a summed-area table in the format of another `Image` of the same type and dimensions
/// as `input`
pub fn generate_summed_area_table(input: &Image<f32>) -> Image<f32> {
    let mut output = Image::blank(input.info());
    let (width, height, channels) = input.info().whc();
    let zeroes = vec![0.0; channels as usize];

    for y in 0..height {
        for x in 0..width {
            let p_in = input.get_pixel(x, y);
            let mut p_out = Vec::new();
            let mut p_top = zeroes.as_slice();
            let mut p_left = zeroes.as_slice();
            let mut p_diag = zeroes.as_slice();

            if x > 0 {
                p_left = output.get_pixel(x - 1, y);

                if y > 0 {
                    p_diag = output.get_pixel(x - 1, y - 1);
                }
            }
            if y > 0 {
                p_top = output.get_pixel(x, y - 1);
            }

            for c in 0..(channels as usize) {
                p_out.push(p_in[c] + p_top[c] + p_left[c] - p_diag[c]);
            }

            output.set_pixel(x, y, &p_out);
        }
    }

    output
}

/// Computes the sum of pixel intensities over a rectangular region with top left corner located
/// at `(x_0, y_0)` and bottom right corner located at `(x_1, y_1)`
pub fn rectangular_intensity_sum(summed_area_table: &Image<f32>, x_0: u32, y_0: u32, x_1: u32, y_1: u32) -> Vec<f32> {
    let channels = summed_area_table.info().channels as usize;
    let mut sum = Vec::new();

    let zeroes = vec![0.0; channels];
    let mut top_left = zeroes.as_slice();
    let mut top_right = zeroes.as_slice();
    let mut bot_left = zeroes.as_slice();
    let bot_right = summed_area_table.get_pixel(x_1, y_1);

    if x_0 != 0 {
        bot_left = summed_area_table.get_pixel(x_0 - 1, y_1);

        if y_0 != 0 {
            top_left = summed_area_table.get_pixel(x_0 - 1, y_0 - 1);
        }
    }
    if y_0 != 0 {
        top_right = summed_area_table.get_pixel(x_1, y_0 - 1);
    }

    for i in 0..channels {
        sum.push(bot_right[i] - top_right[i] - bot_left[i] + top_left[i]);
    }

    sum
}

/// Converts 1D vector index to 2D matrix coordinates
pub fn get_2d_coords(i: u32, width: u32) -> (u32, u32) {
    let x = i % width;
    let y = (i - x) / width;

    (x, y)
}