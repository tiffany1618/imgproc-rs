use crate::util;
use crate::error::ImgProcResult;
use crate::image::{Image, BaseImage};

use rayon::prelude::*;

use std::collections::HashMap;

type HistMap = Vec<Vec<HashMap<u8, i32>>>;
type HistArray = Vec<Vec<[u8; 256]>>;

// struct HistBTreeMap<T: Number> {
//     data: Vec<Vec<BTreeMap<T, >>>
// }

/// Applies a median filter, where each output pixel is the median of the pixels in a
/// `(2 * radius + 1) x (2 * radius + 1)` kernel in the input image
pub fn median_filter(input: &Image<f64>, radius: u32) -> ImgProcResult<Image<f64>> {
    let size = 2 * radius + 1;
    let (width, height) = input.info().wh();
    let mut output = Image::blank(input.info());

    for y in 0..height {
        for x in 0..width {
            let p_out = median_pixel(input, size, x, y);
            output.set_pixel(x, y, &p_out);
        }
    }

    Ok(output)
}

/// (Parallel) Applies a median filter, where each output pixel is the median of the pixels in a
/// `(2 * radius + 1) x (2 * radius + 1)` kernel in the input image
pub fn median_filter_par(input: &Image<f64>, radius: u32) -> ImgProcResult<Image<f64>> {
    let size = 2 * radius + 1;
    let (width, height, channels, alpha) = input.info().whca();

    let data: Vec<Vec<f64>> = (0..input.info().size())
        .into_par_iter()
        .map(|i| {
            let (x, y) = util::get_2d_coords(i, width);
            median_pixel(input, size, x, y)
        })
        .collect();

    Ok(Image::from_vec_of_vec(width, height, channels, alpha, data))
}

fn median_pixel(input: &Image<f64>, size: u32, x: u32, y: u32) -> Vec<f64> {
    let center = ((size * size) / 2) as usize;
    let pixels = input.get_neighborhood_2d(x, y, size);
    let mut p_out = Vec::with_capacity(input.info().channels as usize);

    for c in 0..(input.info().channels as usize) {
        let mut p_in = Vec::with_capacity(input.info().channels as usize);

        for i in 0..((size * size) as usize) {
            p_in.push(pixels[i][c]);
        }

        p_in.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        p_out.push(p_in[center]);
    }

    p_out
}

pub fn median_filter_weiss(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    let size = 2 * radius + 1;
    let center = ((size * size) / 2) as i32;
    let (width, height, channels) = input.info().whc();

    let mut output = Image::blank(input.info());
    let mut histograms: HistMap = vec![vec![HashMap::new(); (height + 2 * radius) as usize]; channels as usize];

    // Initialize histograms
    for x in -(radius as i32)..(radius as i32 + 1) {
        for y in -(radius as i32)..((height + radius) as i32) {
            let p_in = input.get_pixel_unchecked(x as u32, y as u32);

            for c in 0..(channels as usize) {
                *histograms[c][(y + radius as i32) as usize].entry(p_in[c]).or_insert(0) += 1;
            }
        }
    }

    // Compute first column of median values
    median_col(&mut output, &histograms, radius, center, 0);

    // Compute remaining median values
    for x in 1..width {
        // Update histograms
        for y in -(radius as i32)..((height + radius) as i32) {
            let y_index = (y + radius as i32) as usize;
            let p_rem = input.get_pixel_unchecked((x as i32 - radius as i32 - 1) as u32, y as u32);
            let p_add = input.get_pixel_unchecked(x + radius, y as u32);

            for c in 0..(channels as usize) {
                // Remove old pixel value
                match histograms[c][y_index].get_mut(&p_rem[c]) {
                    Some(v) => {
                        if *v == 1 {
                            histograms[c][y_index].remove(&p_rem[c]);
                        } else {
                            *v -= 1;
                        }
                    }
                    None => {},
                }

                // Add new pixel value
                *histograms[c][y_index].entry(p_add[c]).or_insert(0) += 1;
            }
        }

        // Compute median values
        median_col(&mut output, &histograms, radius, center, x);
    }

    Ok(output)
}

fn median_col(output: &mut Image<u8>, histograms: &HistMap, radius: u32, center: i32, x: u32) {
    let channels = output.info().channels as usize;

    for y in 0..output.info().height {
        let mut p_out = Vec::with_capacity(channels);
        let mut sums = vec![0; channels];

        'outer: for c in 0..channels {
            for key in 0u8..=255 {
                for i in y..(y + 2 * radius + 1) {
                    match histograms[c][i as usize].get(&key) {
                        Some(v) => {
                            sums[c] += v;

                            if sums[c] >= center {
                                p_out.push(key);
                                continue 'outer;
                            }
                        },
                        None => {},
                    }
                }
            }
        }

        output.set_pixel(x, y, &p_out);
    }
}

pub fn median_filter_weiss_arr(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    let size = 2 * radius + 1;
    let center = ((size * size) / 2 + 1) as u8 ;
    let (width, height, channels) = input.info().whc();

    let mut output = Image::blank(input.info());
    let mut histograms: HistArray = vec![vec![[0; 256]; (height + 2 * radius) as usize]; channels as usize];

    // Initialize histograms
    for x in -(radius as i32)..(radius as i32 + 1) {
        for y in -(radius as i32)..((height + radius) as i32) {
            let p_in = input.get_pixel_unchecked(x.clamp(0, width as i32 - 1) as u32,
                                                 y.clamp(0, height as i32 - 1) as u32);

            for c in 0..(channels as usize) {
                histograms[c][(y + radius as i32) as usize][p_in[c] as usize] += 1;
            }
        }
    }

    // Compute first column of median values
    median_col_arr(&mut output, &histograms, radius, center, 0);

    // Compute remaining median values
    for x in 1..width {
        let x_rem = (x as i32 - radius as i32 - 1).clamp(0, width as i32 - 1) as u32;
        let x_add = (x + radius).clamp(0, width - 1);

        // Update histograms
        for y in -(radius as i32)..((height + radius) as i32) {
            let y_index = (y + radius as i32) as usize;
            let y_clamp = y.clamp(0, height as i32 - 1) as u32;

            let p_rem = input.get_pixel_unchecked(x_rem, y_clamp);
            let p_add = input.get_pixel_unchecked(x_add, y_clamp);

            for c in 0..(channels as usize) {
                // Remove old pixel value
                histograms[c][y_index][p_rem[c] as usize] -= 1;

                // Add new pixel value
                histograms[c][y_index][p_add[c] as usize] += 1;
            }
        }

        // Compute median values
        median_col_arr(&mut output, &histograms, radius, center, x);
    }

    Ok(output)
}

fn median_col_arr(output: &mut Image<u8>, histograms: &HistArray, radius: u32, center: u8, x: u32) {
    if x < 5 {
        println!("x: {}", x);
        for c in 0..output.info().channels {
            println!("c: {}", c);
            for i in 0..10 {
                for u in 0..=255 {
                    if histograms[c as usize][i][u] != 0 {
                        print!("({}: {}), ", u, histograms[c as usize][i][u]);
                    }
                }
                println!("---");
            }
        }
    }

    let channels = output.info().channels as usize;
    let mut sums = vec![0; channels];

    // Initialize central histogram and find first median value
    let mut p_prev = Vec::with_capacity(channels);
    'outer_1: for c in 0..channels {
        for key in 0u8..=255 {
            for i in 0..(2 * radius + 1) {
                if sums[c] + histograms[c][i as usize][key as usize] >= center {
                    p_prev.push(key);
                    continue 'outer_1;
                }

                sums[c] += histograms[c][i as usize][key as usize];
            }
        }
    }
    output.set_pixel(x, 0, &p_prev);

    for y in 1..output.info().height {
        let y_rem = (y as i32 - 1).clamp(0, output.info().height as i32 - 1) as usize;
        let y_add = (y + 2 * radius) as usize;

        let mut p_out = Vec::with_capacity(channels);
        'outer_2: for c in 0..channels {
            for key in 0u8..p_prev[c] {
                // Remove old pixel values
                sums[c] -= histograms[c][y_rem][key as usize];

                // Add new pixel values
                sums[c] += histograms[c][y_add][key as usize];
            }

            if sums[c] < center {
                for key in p_prev[c]..=255 {
                    for i in y..(y + 2 * radius + 1) {
                        if sums[c] + histograms[c][i as usize][key as usize] >= center {
                            p_out.push(key);
                            continue 'outer_2;
                        }

                        sums[c] += histograms[c][i as usize][key as usize];
                    }
                }
            } else {
                for key in (0..p_prev[c]).rev() {
                    for i in y..(y + 2 * radius + 1) {
                        sums[c] -= histograms[c][i as usize][key as usize];

                        if sums[c] < center {
                            p_out.push(key);
                            continue 'outer_2;
                        }
                    }
                }
            }
        }

        output.set_pixel(x, y, &p_out);
        p_prev = p_out;
    }
}

pub fn median_filter_huang(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    let size = 2 * radius + 1;
    let center = ((size * size) / 2 + 1) as u8 ;
    let (width, height, channels) = input.info().whc();

    let mut output = Image::blank(input.info());
    let mut histograms  = vec![[0; 256]; channels as usize];

    // Initialize histograms
    for x in -(radius as i32)..(radius as i32 + 1) {
        for y in -(radius as i32)..(radius as i32 + 1) {
            let p_in = input.get_pixel_unchecked(x.clamp(0, width as i32 - 1) as u32,
                                                 y.clamp(0, height as i32 - 1) as u32);

            for c in 0..(channels as usize) {
                histograms[c][p_in[c] as usize] += 1;
            }
        }
    }

    // Compute first column of median values
    median_col_huang(&mut output, &histograms, radius, center, 0);

    // Compute remaining median values
    for x in 1..width {
        let x_rem = (x as i32 - radius as i32 - 1).clamp(0, width as i32 - 1) as u32;
        let x_add = (x + radius).clamp(0, width - 1);

        // Update histograms
        for y in -(radius as i32)..((height + radius) as i32) {
            let y_index = (y + radius as i32) as usize;
            let y_clamp = y.clamp(0, height as i32 - 1) as u32;

            let p_rem = input.get_pixel_unchecked(x_rem, y_clamp);
            let p_add = input.get_pixel_unchecked(x_add, y_clamp);

            for c in 0..(channels as usize) {
                // Remove old pixel value
                histograms[c][y_index][p_rem[c] as usize] -= 1;

                // Add new pixel value
                histograms[c][y_index][p_add[c] as usize] += 1;
            }
        }

        // Compute median values
        median_col_huang(&mut output, &histograms, radius, center, x);
    }

    Ok(output)
}

fn median_col_huang(output: &mut Image<u8>, histograms: &Vec<[u32; 256]>, radius: u32, center: u8, x: u32) {
    let channels = output.info().channels as usize;
    let mut sums = vec![0; channels];

    // Find first median value
    let mut p_prev = Vec::with_capacity(channels);
    'outer_1: for c in 0..channels {
        for key in 0u8..=255 {
            if sums[c] + histograms[c][key as usize] >= center {
                p_prev.push(key);
                continue 'outer_1;
            }

            sums[c] += histograms[c][i as usize][key as usize];
        }
    }
    output.set_pixel(x, 0, &p_prev);

    for y in 1..output.info().height {
        let y_rem = (y as i32 - 1).clamp(0, output.info().height as i32 - 1) as usize;
        let y_add = (y + 2 * radius) as usize;

        let mut p_out = Vec::with_capacity(channels);
        'outer_2: for c in 0..channels {
            for key in 0u8..p_prev[c] {
                // Remove old pixel values
                sums[c] -= histograms[c][y_rem][key as usize];

                // Add new pixel values
                sums[c] += histograms[c][y_add][key as usize];
            }

            if sums[c] < center {
                for key in p_prev[c]..=255 {
                    for i in y..(y + 2 * radius + 1) {
                        if sums[c] + histograms[c][i as usize][key as usize] >= center {
                            p_out.push(key);
                            continue 'outer_2;
                        }

                        sums[c] += histograms[c][i as usize][key as usize];
                    }
                }
            } else {
                for key in (0..p_prev[c]).rev() {
                    for i in y..(y + 2 * radius + 1) {
                        sums[c] -= histograms[c][i as usize][key as usize];

                        if sums[c] < center {
                            p_out.push(key);
                            continue 'outer_2;
                        }
                    }
                }
            }
        }

        output.set_pixel(x, y, &p_out);
        p_prev = p_out;
    }
}