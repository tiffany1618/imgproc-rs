use crate::util;
use crate::error::ImgProcResult;
use crate::image::{Image, BaseImage, Number};

use rayon::prelude::*;

/// Applies a median filter, where each output pixel is the median of the pixels in a
/// `(2 * radius + 1) x (2 * radius + 1)` kernel in the input image
pub fn median_filter<T: Number>(input: &Image<T>, radius: u32) -> ImgProcResult<Image<T>> {
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
pub fn median_filter_par<T: Number>(input: &Image<T>, radius: u32) -> ImgProcResult<Image<T>> {
    let size = 2 * radius + 1;
    let (width, height, channels, alpha) = input.info().whca();

    let data: Vec<Vec<T>> = (0..input.info().size())
        .into_par_iter()
        .map(|i| {
            let (x, y) = util::get_2d_coords(i, width);
            median_pixel(input, size, x, y)
        })
        .collect();

    Ok(Image::from_vec_of_vec(width, height, channels, alpha, data))
}

fn median_pixel<T: Number>(input: &Image<T>, size: u32, x: u32, y: u32) -> Vec<T> {
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

////////////////////////
// Weiss Median Filter
////////////////////////

/// Applies a median filter, using Weiss' partial histogram method with a tier radix of 2
/// For a detailed description, see:
/// http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.93.1608&rep=rep1&type=pdf
pub fn median_filter_weiss(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    let mut n_cols = (4.0 * (radius as f64).powf(2.0 / 3.0)).floor() as usize;
    if n_cols % 2 == 0 {
        n_cols += 1;
    }

    let mut output = Image::blank(input.info());

    for x in (0..output.info().width).step_by(n_cols) {
        process_cols(input, &mut output, radius, n_cols, x);
    }

    Ok(output)
}

#[derive(Debug, Clone)]
struct PartialHistograms {
    data: Vec<[i32; 256]>,
    n_cols: usize,
    n_half: usize,
    radius: usize,
    size: usize,
}

impl PartialHistograms {
    pub fn new(radius: usize, n_cols: usize) -> Self {
        let size = (2 * radius + 1) as usize;
        let n_half = n_cols / 2;

        PartialHistograms {
            data: vec![[0; 256]; n_cols],
            n_cols,
            n_half,
            radius,
            size,
        }
    }

    pub fn edit_row(&mut self, p_in: &Vec<&[u8]>, channel_index: usize, add: bool) {
        let mut inc = 1;
        if !add {
            inc *= -1;
        }

        for n in 0..self.n_half {
            let n_upper = self.n_cols - n - 1;

            for i in n..self.n_half {
                self.data[n][p_in[i][channel_index] as usize] += inc;
                self.data[n][p_in[i+self.size][channel_index] as usize] -= inc;

                let i_upper = self.n_cols + 2 * self.radius - i - 1;
                let i_lower = i_upper - self.size;
                self.data[n_upper][p_in[i_lower][channel_index] as usize] -= inc;
                self.data[n_upper][p_in[i_upper][channel_index] as usize] += inc;
            }
        }

        for i in self.n_half..(self.n_half + self.size) {
            self.data[self.n_half][p_in[i][channel_index] as usize] += inc;
        }
    }

    pub fn central_hist(&self) -> &[i32; 256] {
        &self.data[self.n_half]
    }

    pub fn partial_hist(&self, index: usize) -> &[i32; 256] {
        &self.data[index]
    }
}

fn add_row(histograms: &mut Vec<PartialHistograms>, p_in: &Vec<&[u8]>, channels: usize) {
    for c in 0..channels {
        histograms[c].edit_row(p_in, c, true);
    }
}

fn remove_row(histograms: &mut Vec<PartialHistograms>, p_in: &Vec<&[u8]>, channels: usize) {
    for c in 0..channels {
        histograms[c].edit_row(p_in, c, false);
    }
}

fn process_cols(input: &Image<u8>, output: &mut Image<u8>, radius: u32, n_cols: usize, x: u32) {
    let size = 2 * radius + 1;
    let center = ((size * size) / 2 + 1) as i32;
    let (width, height, channels) = input.info().whc();
    let mut histograms = vec![PartialHistograms::new(radius as usize, n_cols); channels as usize];

    // Initialize histograms
    for j in -(radius as i32)..(radius as i32 + 1) {
        let mut p_in = Vec::new();
        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            p_in.push(input.get_pixel_unchecked(i.clamp(0, width as i32 - 1) as u32,
                                                j.clamp(0, height as i32 - 1) as u32));
        }

        add_row(&mut histograms, &p_in, channels as usize);
    }

    // Compute first median values
    process_row(output, &histograms, center, n_cols, x, 0);

    // Compute remaining median values
    for j in 1..height {
        // Update histograms
        let mut p_in = Vec::new();
        let mut p_out = Vec::new();
        let j_in = (j + radius).clamp(0, input.info().height - 1);
        let j_out = (j as i32 - radius as i32 - 1).clamp(0, input.info().height as i32 - 1) as u32;

        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            let i_clamp = i.clamp(0, width as i32 - 1) as u32;
            p_in.push(input.get_pixel_unchecked(i_clamp, j_in));
            p_out.push(input.get_pixel_unchecked(i_clamp, j_out));
        }

        add_row(&mut histograms, &p_in, channels as usize);
        remove_row(&mut histograms, &p_out, channels as usize);

        process_row(output, &histograms, center, n_cols, x, j);
    }
}

fn process_row(output: &mut Image<u8>, histograms: &Vec<PartialHistograms>, center: i32, n_cols: usize, x: u32, y: u32) {
    let channels = output.info().channels as usize;

    for i in 0..n_cols {
        let mut p_out = Vec::new();
        for c in 0..channels {
            let partial = histograms[c].partial_hist(i);
            let mut sum = 0;

            for key in 0u8..=255 {
                sum += histograms[c].central_hist()[key as usize];

                if i != n_cols / 2 {
                    sum += partial[key as usize];
                }

                if sum >= center {
                    p_out.push(key);
                    break;
                }
            }
        }

        let x_clamp = (x + i as u32).clamp(0, output.info().width - 1);
        output.set_pixel(x_clamp, y, &p_out);
    }
}