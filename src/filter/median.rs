use crate::{util, error};
use crate::error::{ImgProcResult, ImgProcError};
use crate::image::{Image, BaseImage, Number};

use rayon::prelude::*;

/// Applies a median filter, where each output pixel is the median of the pixels in a
/// `(2 * radius + 1) x (2 * radius + 1)` kernel in the input image. Based on Ben Weiss' partial
/// histogram method, using a tier radix of 2. For a detailed description, see:
/// http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.93.1608&rep=rep1&type=pdf
pub fn median_filter(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    let mut n_cols = (4.0 * (radius as f64).powf(2.0 / 3.0)).floor() as usize;
    if n_cols % 2 == 0 {
        n_cols += 1;
    }

    let mut output = Image::blank(input.info());

    for x in (0..output.info().width).step_by(n_cols) {
        process_cols_med(input, &mut output, radius, n_cols, x);
    }

    Ok(output)
}

// /// Applies an alpha-trimmed mean filter, where each output pixel is the alpha-trimmed mean of the
// /// pixels in a `(2 * radius + 1) x (2 * radius + 1)` kernel in the input image
// pub fn alpha_trimmed_mean(input: &Image<u8>, radius: u32, alpha: u32) -> ImgProcResult<Image<u8>> {
//     let size = 2 * radius + 1;
//     error::check_even(alpha, "alpha")?;
//     if alpha >= (size * size) {
//         return Err(ImgProcError::InvalidArgError(format!("invalid alpha: size is {}, but alpha is {}", size, alpha)));
//     }
//
//     let mut n_cols = (4.0 * (radius as f64).powf(2.0 / 3.0)).floor() as usize;
//     if n_cols % 2 == 0 {
//         n_cols += 1;
//     }
//
//     let mut output = Image::blank(input.info());
//
//     for x in (0..output.info().width).step_by(n_cols) {
//         process_cols_mean(input, &mut output, radius, alpha, n_cols, x);
//     }
//
//     Ok(output)
// }

#[derive(Debug, Clone)]
struct PartialHistograms {
    data: Vec<[i32; 256]>, // The partial histograms
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

    pub fn update(&mut self, p_in: &Vec<&[u8]>, channel_index: usize, add: bool) {
        let mut inc = 1;
        if !add {
            inc *= -1;
        }

        // Update partial histograms
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

        // Update central histogram
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

////////////////////////////
// Median filter functions
////////////////////////////

#[derive(Debug, Clone)]
struct MedianHist {
    data: PartialHistograms,
    sums: Vec<i32>, // Sums to keep track of the number of values less than the previous median
    pivots: Vec<u8>, // Previous medians to act as "pivots" to find the next median
}

impl MedianHist {
    pub fn new(radius: usize, n_cols: usize) -> Self {
        MedianHist {
            data: PartialHistograms::new(radius, n_cols),
            sums: vec![0; n_cols],
            pivots: Vec::with_capacity(n_cols),
        }
    }

    pub fn data(&self) -> &PartialHistograms {
        &self.data
    }

    pub fn sums(&self) -> &[i32] {
        &self.sums
    }

    pub fn pivots(&self) -> &[u8] {
        &self.pivots
    }

    pub fn init_pivots(&mut self) {
        self.pivots = vec![0; self.data.n_cols];
    }

    pub fn set_pivot(&mut self, pivot: u8, index: usize) {
        self.pivots[index] = pivot;
    }

    pub fn set_sum(&mut self, sum: i32, index: usize) {
        self.sums[index] = sum;
    }

    pub fn update(&mut self, p_in: &Vec<&[u8]>, channel_index: usize, add: bool) {
        self.data.update(p_in, channel_index, add);

        let mut inc = 1;
        if !add {
            inc *= -1;
        }

        // Update sums
        if !self.pivots.is_empty() {
            for n in 0..self.data.n_cols {
                for i in n..(n + self.data.size) {
                    if p_in[i][channel_index] < self.pivots[n] {
                        self.sums[n] += inc;
                    }
                }
            }
        }
    }
}

fn process_cols_med(input: &Image<u8>, output: &mut Image<u8>, radius: u32, n_cols: usize, x: u32) {
    let size = 2 * radius + 1;
    let center = ((size * size) / 2 + 1) as i32;
    let (width, height, channels) = input.info().whc();
    let mut histograms = vec![MedianHist::new(radius as usize, n_cols); channels as usize];

    // Initialize histogram and process first row
    init_cols_med(input, output, &mut histograms, radius, center, n_cols, x);

    // Update histogram and process remaining rows
    for j in 1..height {
        // Update histograms
        let mut p_in = Vec::with_capacity(n_cols);
        let mut p_out = Vec::with_capacity(n_cols);
        let j_in = (j + radius).clamp(0, input.info().height - 1);
        let j_out = (j as i32 - radius as i32 - 1).clamp(0, input.info().height as i32 - 1) as u32;

        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            let i_clamp = i.clamp(0, width as i32 - 1) as u32;
            p_in.push(input.get_pixel_unchecked(i_clamp, j_in));
            p_out.push(input.get_pixel_unchecked(i_clamp, j_out));
        }

        add_row_med(&mut histograms, &p_in, channels as usize);
        remove_row_med(&mut histograms, &p_out, channels as usize);

        process_row_med(output, &mut histograms, center, n_cols, x, j);
    }
}

fn init_cols_med(input: &Image<u8>, output: &mut Image<u8>, histograms: &mut Vec<MedianHist>, radius: u32, center: i32, n_cols: usize, x: u32) {
    let (width, height, channels) = input.info().whc();

    // Initialize histograms
    for j in -(radius as i32)..(radius as i32 + 1) {
        let mut p_in = Vec::with_capacity(n_cols);
        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            p_in.push(input.get_pixel_unchecked(i.clamp(0, width as i32 - 1) as u32,
                                                j.clamp(0, height as i32 - 1) as u32));
        }

        add_row_med(histograms, &p_in, channels as usize);
    }

    // Initialize histogram pivots
    for c in 0..(channels as usize) {
        histograms[c].init_pivots();
    }

    // Compute first median values
    for i in 0..n_cols {
        let mut p_out = Vec::with_capacity(channels as usize);
        for c in 0..(channels as usize) {
            let partial = histograms[c].data().partial_hist(i);
            let mut sum = 0;

            for key in 0u8..=255 {
                let mut add = histograms[c].data().central_hist()[key as usize];
                if i != n_cols / 2 {
                    add += partial[key as usize];
                }

                if sum + add >= center {
                    p_out.push(key);
                    histograms[c].set_sum(sum, i);
                    break;
                }

                sum += add;
            }
        }

        let x_clamp = (x + i as u32).clamp(0, output.info().width - 1);
        output.set_pixel(x_clamp, 0, &p_out);

        set_pivots_med(histograms, &p_out, i);
    }
}

fn process_row_med(output: &mut Image<u8>, histograms: &mut Vec<MedianHist>, center: i32, n_cols: usize, x: u32, y: u32) {
    let channels = output.info().channels as usize;

    for i in 0..n_cols {
        let mut p_out = Vec::with_capacity(channels);
        for c in 0..channels {
            let partial = histograms[c].data().partial_hist(i);
            let pivot = histograms[c].pivots()[i];
            let mut sum = histograms[c].sums()[i];

            if sum < center {
                for key in pivot..=255 {
                    let mut add = histograms[c].data().central_hist()[key as usize];
                    if i != n_cols / 2 {
                        add += partial[key as usize];
                    }

                    if sum + add >= center {
                        p_out.push(key);
                        histograms[c].set_sum(sum, i);
                        break;
                    }

                    sum += add;
                }
            } else {
                for key in (0..pivot).rev() {
                    sum -= partial[key as usize];
                    if i != n_cols / 2 {
                        sum -= histograms[c].data().central_hist()[key as usize];
                    }

                    if sum < center {
                        p_out.push(key);
                        histograms[c].set_sum(sum, i);
                        break;
                    }
                }
            }
        }

        let x_clamp = (x + i as u32).clamp(0, output.info().width - 1);
        output.set_pixel(x_clamp, y, &p_out);

        set_pivots_med(histograms, &p_out, i);
    }
}

fn add_row_med(histograms: &mut Vec<MedianHist>, p_in: &Vec<&[u8]>, channels: usize) {
    for c in 0..channels {
        histograms[c].update(p_in, c, true);
    }
}

fn remove_row_med(histograms: &mut Vec<MedianHist>, p_in: &Vec<&[u8]>, channels: usize) {
    for c in 0..channels {
        histograms[c].update(p_in, c, false);
    }
}

fn set_pivots_med(histograms: &mut Vec<MedianHist>, pivots: &Vec<u8>, index: usize) {
    for c in 0..pivots.len() {
        histograms[c].set_pivot(pivots[c], index);
    }
}

////////////////////////////////////////
// Alpha-trimmed mean filter functions
////////////////////////////////////////

#[derive(Debug, Clone)]
struct MeanHist {
    data: PartialHistograms,
    means: Vec<i32>,
    lower: Vec<u8>,
    upper: Vec<u8>,
}

impl MeanHist {
    pub fn new(radius: usize, n_cols: usize) -> Self {
        MeanHist {
            data: PartialHistograms::new(radius, n_cols),
            means: Vec::with_capacity(n_cols),
            lower: Vec::with_capacity(n_cols),
            upper: Vec::with_capacity(n_cols),
        }
    }

    pub fn data(&self) -> &PartialHistograms {
        &self.data
    }

    pub fn means(&self) -> &[i32] {
        &self.means
    }

    pub fn lower(&self) -> &[u8] {
        &self.lower
    }

    pub fn upper(&self) -> &[u8] {
        &self.upper
    }

    pub fn init(&mut self) {
        self.means = vec![0; self.data.n_cols];
        self.lower = vec![0; self.data.n_cols];
        self.upper = vec![0; self.data.n_cols];
    }

    pub fn update(&mut self, p_in: &Vec<&[u8]>, channel_index: usize, add: bool) {
        self.data.update(p_in, channel_index, add);
    }
}

fn process_cols_mean(input: &Image<u8>, output: &mut Image<u8>, radius: u32, alpha: u32, n_cols: usize, x: u32) {
    let (width, height, channels) = input.info().whc();
    let mut histograms = vec![MeanHist::new(radius as usize, n_cols); channels as usize];

    // Initialize histogram and process first row
    init_cols_mean(input, output, &mut histograms, radius, alpha, n_cols, x);

    // Update histogram and process remaining rows
    for j in 1..height {
        // Update histograms
        let mut p_in = Vec::with_capacity(n_cols);
        let mut p_out = Vec::with_capacity(n_cols);
        let j_in = (j + radius).clamp(0, input.info().height - 1);
        let j_out = (j as i32 - radius as i32 - 1).clamp(0, input.info().height as i32 - 1) as u32;

        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            let i_clamp = i.clamp(0, width as i32 - 1) as u32;
            p_in.push(input.get_pixel_unchecked(i_clamp, j_in));
            p_out.push(input.get_pixel_unchecked(i_clamp, j_out));
        }

        add_row_mean(&mut histograms, &p_in, channels as usize);
        remove_row_mean(&mut histograms, &p_out, channels as usize);

        process_row_mean(output, &mut histograms, n_cols, x, j);
    }
}

fn init_cols_mean(input: &Image<u8>, output: &mut Image<u8>, histograms: &mut Vec<MeanHist>, radius: u32, alpha: u32, n_cols: usize, x: u32) {
    let (width, height, channels) = input.info().whc();

    // Initialize histograms
    for j in -(radius as i32)..(radius as i32 + 1) {
        let mut p_in = Vec::with_capacity(n_cols);
        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            p_in.push(input.get_pixel_unchecked(i.clamp(0, width as i32 - 1) as u32,
                                                j.clamp(0, height as i32 - 1) as u32));
        }

        add_row_mean(histograms, &p_in, channels as usize);
    }

    // Initialize histograms
    for c in 0..(channels as usize) {
        histograms[c].init();
    }

    // Compute first mean values
    for i in 0..n_cols {
        let mut p_out = Vec::with_capacity(channels as usize);
        for c in 0..(channels as usize) {
            let partial = histograms[c].data().partial_hist(i);
            let mut sum = 0;
            let mut lower = 255u8;
            let mut upper = 0u8;

            for key in 0u8..=255 {
                let mut add = histograms[c].data().central_hist()[key as usize];
                if i != n_cols / 2 {
                    add += partial[key as usize];
                }

                sum += add;
            }
        }

        let x_clamp = (x + i as u32).clamp(0, output.info().width - 1);
        output.set_pixel(x_clamp, 0, &p_out);
    }
}

fn process_row_mean(output: &mut Image<u8>, histograms: &mut Vec<MeanHist>, n_cols: usize, x: u32, y: u32) {
    let channels = output.info().channels as usize;

    for i in 0..n_cols {
        let mut p_out = Vec::with_capacity(channels);
        for c in 0..channels {
            let partial = histograms[c].data().partial_hist(i);

        }

        let x_clamp = (x + i as u32).clamp(0, output.info().width - 1);
        output.set_pixel(x_clamp, y, &p_out);
    }
}

fn add_row_mean(histograms: &mut Vec<MeanHist>, p_in: &Vec<&[u8]>, channels: usize) {
    for c in 0..channels {
        histograms[c].update(p_in, c, true);
    }
}

fn remove_row_mean(histograms: &mut Vec<MeanHist>, p_in: &Vec<&[u8]>, channels: usize) {
    for c in 0..channels {
        histograms[c].update(p_in, c, false);
    }
}