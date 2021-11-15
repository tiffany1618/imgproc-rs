use crate::error;
use crate::error::{ImgProcResult, ImgProcError};
use crate::image::{Image, BaseImage};

use std::cmp::{Ordering, Reverse};

/// Applies a median filter, where each output pixel is the median of the pixels in a
/// `(2 * radius + 1) x (2 * radius + 1)` kernel in the input image. Based on Ben Weiss' partial
/// histogram method, using a tier radix of 2. A detailed description can be found
/// [here](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.93.1608&rep=rep1&type=pdf).
pub fn median_filter(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    let mut n_cols = (4.0 * (radius as f32).powf(2.0 / 3.0)).floor() as usize;
    if n_cols % 2 == 0 {
        n_cols += 1;
    }

    let mut output = Image::blank(input.info());

    for x in (0..output.info().width).step_by(n_cols) {
        process_cols_med(input, &mut output, radius, n_cols, x);
    }

    Ok(output)
}

/// Applies an alpha-trimmed mean filter, where each output pixel is the mean of the
/// pixels in a `(2 * radius + 1) x (2 * radius + 1)` kernel in the input image, with the lowest
/// `alpha / 2` pixels and the highest `alpha / 2` pixels removed.
pub fn alpha_trimmed_mean_filter(input: &Image<u8>, radius: u32, alpha: u32) -> ImgProcResult<Image<u8>> {
    let size = 2 * radius + 1;
    error::check_even(alpha, "alpha")?;
    if alpha >= (size * size) {
        return Err(ImgProcError::InvalidArgError(format!("invalid alpha: size is {}, but alpha is {}", size, alpha)));
    }

    let mut n_cols = (4.0 * (radius as f32).powf(2.0 / 3.0)).floor() as usize;
    if n_cols % 2 == 0 {
        n_cols += 1;
    }

    let mut output = Image::blank(input.info());

    for x in (0..output.info().width).step_by(n_cols) {
        process_cols_mean(input, &mut output, radius, alpha, n_cols, x);
    }

    Ok(output)
}

/*
 * The PartialHistograms struct:
 *
 * This struct contains the partial histograms, which is a vector of an odd number of histograms
 * determined by n_cols. The only "complete" histogram is the central histogram (located at
 * data[n_half]), which is the histogram of the pixel values in the kernel surrounding the
 * central pixel in the row that is being processed. Each histogram to the left and right of the
 * central histogram is not another "complete" histogram, but rather is a histogram representing
 * the difference between the histogram for the pixel at that location and the central histogram.
 * As such, the values in these "partial" histograms can (and frequently will) be negative.
 * The "complete" histogram for each non-central pixel is then just the sum of the corresponding
 * partial histogram and the central histogram.
 *
 * Algorithm overview:
 *
 * The basic idea of this algorithm is to process a row of n_cols pixels at once using the
 * partial histograms to efficiently compute the complete histograms for each pixel. To process the
 * next row, the partial histograms are updated to remove the top row of pixel values from the
 * previous kernel and add the bottom row of pixel values from the current kernel. Each set of
 * n_cols columns in the image is processed in this fashion, using a single set of partial
 * histograms that are updated as the current kernel slides down the image.
 */
#[derive(Debug, Clone)]
struct PartialHistograms {
    data: Vec<[i32; 256]>, // The partial histograms
    n_cols: usize, // The number of partial histograms, which is always odd. This also denotes the
                   // number of columns we can process at once
    n_half: usize, // Half the number of partial histograms, rounded down
    radius: usize, // The radius of the kernel we are using
    size: usize, // The number of pixels in a kernel
}

impl PartialHistograms {
    fn new(radius: usize, n_cols: usize) -> Self {
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

    // Add or remove a row of pixels from the histograms, as indicated by the add parameter
    fn update(&mut self, p_in: &[&[u8]], channel_index: usize, add: bool) {
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

    // Returns the number of pixels with a value of key in the kernel for the pixel at the given
    // index
    fn get_count(&self, key: usize, index: usize) -> i32 {
        let mut count = self.data[self.n_half][key as usize];
        if index != self.n_half {
            count += self.data[index][key as usize];
        }

        count as i32
    }
}

////////////////////////////
// Median filter functions
////////////////////////////

/*
 * The MedianHist struct:
 *
 * In addition to containing the partial histograms, this struct keeps track of each previous
 * median of the kernel for each pixel. These medians are used as "pivots" to find the next
 * median: to find the median of the kernel for a given pixel, instead of scanning its histogram
 * starting from 0, we start from the median of the kernel of the previous pixel in that column.
 * This value is typically much closer to the current median since the majority of the pixels
 * in the previous and current kernels are the same, which makes scanning the histogram much
 * quicker. The "sums", or the number of values in the current histogram that are less than the
 * previous median, is used to determine if the current median is greater than or less than the
 * previous median, thus determining if the current histogram should be scanned upwards or
 * downwards from the previous median, respectively, to find the current median.
 */
#[derive(Debug, Clone)]
struct MedianHist {
    data: PartialHistograms,
    sums: Vec<i32>, // Sums to keep track of the number of values less than the previous median
    pivots: Vec<u8>, // Previous medians to act as "pivots" to find the next median
}

impl MedianHist {
    fn new(radius: usize, n_cols: usize) -> Self {
        MedianHist {
            data: PartialHistograms::new(radius, n_cols),
            sums: vec![0; n_cols],
            pivots: Vec::with_capacity(n_cols),
        }
    }

    fn data(&self) -> &PartialHistograms {
        &self.data
    }

    fn sums(&self) -> &[i32] {
        &self.sums
    }

    fn pivots(&self) -> &[u8] {
        &self.pivots
    }

    fn init_pivots(&mut self) {
        self.pivots = vec![0; self.data.n_cols];
    }

    fn set_pivot(&mut self, pivot: u8, index: usize) {
        self.pivots[index] = pivot;
    }

    fn set_sum(&mut self, sum: i32, index: usize) {
        self.sums[index] = sum;
    }

    fn update(&mut self, p_in: &[&[u8]], channel_index: usize, add: bool) {
        self.data.update(p_in, channel_index, add);

        let mut inc = 1;
        if !add {
            inc *= -1;
        }

        // Update the number of values less than the previous median
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
    let center = ((size * size) / 2 + 1) as i32; // Half the number of pixels in a kernel. If
                                                      // all the pixels in the kernel were sorted,
                                                      // the index of the median would be (center - 1).
    let (width, height, channels) = input.info().whc();
    let mut histograms = vec![MedianHist::new(radius as usize, n_cols); channels as usize];
    let mut p_out = Vec::with_capacity(channels as usize);

    // Initialize histogram and process first row
    init_cols_med(input, output, &mut histograms, &mut p_out, radius, center, n_cols, x);

    // Update histogram and process remaining rows
    let mut row_in = Vec::with_capacity(n_cols);
    let mut row_out = Vec::with_capacity(n_cols);
    for j in 1..height {
        // Update histograms
        let j_in = (j + radius).clamp(0, input.info().height - 1);
        let j_out = (j as i32 - radius as i32 - 1).clamp(0, input.info().height as i32 - 1) as u32;

        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            let i_clamp = i.clamp(0, width as i32 - 1) as u32;
            row_in.push(input.get_pixel_unchecked(i_clamp, j_in));
            row_out.push(input.get_pixel_unchecked(i_clamp, j_out));
        }

        add_row_med(&mut histograms, &row_in);
        remove_row_med(&mut histograms, &row_out);

        process_row_med(output, &mut histograms, &mut p_out, center, n_cols, x, j);

        row_in.clear();
        row_out.clear();
    }
}

fn init_cols_med(input: &Image<u8>, output: &mut Image<u8>, histograms: &mut Vec<MedianHist>,
                 p_out: &mut Vec<u8>, radius: u32, center: i32, n_cols: usize, x: u32) {
    let (width, height) = input.info().wh();

    // Initialize histograms
    let mut row_in = Vec::with_capacity(n_cols);
    for j in -(radius as i32)..(radius as i32 + 1) {
        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            row_in.push(input.get_pixel_unchecked(i.clamp(0, width as i32 - 1) as u32,
                                                  j.clamp(0, height as i32 - 1) as u32));
        }

        add_row_med(histograms, &row_in);
        row_in.clear();
    }

    // Initialize histogram pivots
    for hist in histograms.iter_mut() {
        hist.init_pivots();
    }

    // Compute first median values
    for i in 0..n_cols {
        p_out.clear();
        for hist in histograms.iter_mut() {
            let mut sum = 0;

            for key in 0u8..=255 {
                let add = hist.data().get_count(key as usize, i);

                if sum + add >= center {
                    p_out.push(key);
                    hist.set_sum(sum, i);
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

fn process_row_med(output: &mut Image<u8>, histograms: &mut Vec<MedianHist>, p_out: &mut Vec<u8>, center: i32, n_cols: usize, x: u32, y: u32) {
    for i in 0..n_cols {
        p_out.clear();
        for hist in histograms.iter_mut() {
            let pivot = hist.pivots()[i]; // Get the previous median
            let mut sum = hist.sums()[i]; // Get the number of values less than
                                                        // the previous median

            match sum.cmp(&center) {
                Ordering::Equal => { // The current median is equal to the previous median
                    p_out.push(pivot);
                },
                Ordering::Less => { // The current median is greater than the previous median,
                                    // so the histogram should be scanned upwards
                    for key in pivot..=255 {
                        let add = hist.data().get_count(key as usize, i);

                        if sum + add >= center {
                            p_out.push(key);
                            hist.set_sum(sum, i);
                            break;
                        }

                        sum += add;
                    }
                },
                Ordering::Greater => { // The current median is less than the previous median, so the histogram
                                       // should be scanned downwards
                    for key in (0..pivot).rev() {
                        sum -= hist.data().get_count(key as usize, i);

                        if sum < center {
                            p_out.push(key);
                            hist.set_sum(sum, i);
                            break;
                        }
                    }
                }
            }
        }

        let x_clamp = (x + i as u32).clamp(0, output.info().width - 1);
        output.set_pixel(x_clamp, y, &p_out);

        set_pivots_med(histograms, &p_out, i);
    }
}

fn add_row_med(histograms: &mut Vec<MedianHist>, p_in: &[&[u8]]) {
    for (c, hist) in histograms.iter_mut().enumerate() {
        hist.update(p_in, c, true);
    }
}

fn remove_row_med(histograms: &mut Vec<MedianHist>, p_in: &[&[u8]]) {
    for (c, hist) in histograms.iter_mut().enumerate() {
        hist.update(p_in, c, false);
    }
}

fn set_pivots_med(histograms: &mut Vec<MedianHist>, pivots: &[u8], index: usize) {
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
    sums: Vec<i32>, // The sum of all the participating pixel values in the kernel for each pixel
    lower: Vec<Vec<u8>>, // Vectors of all the lowest discarded pixel values in the kernel
                         // for each pixel
    upper: Vec<Vec<u8>>, // Vectors of all the highest discarded pixel values in the kernel for
                         // each pixel
    trim: usize, // The number of pixel values discarded at the low and high ends of each kernel
                 // (equal to half of alpha)
    len: f32, // The number of participating pixel values in each kernel
}

impl MeanHist {
    fn new(radius: usize, n_cols: usize, alpha: u32) -> Self {
        let size = 2 * radius + 1;
        let len = ((size * size) - alpha as usize) as f32;

        MeanHist {
            data: PartialHistograms::new(radius, n_cols),
            sums: Vec::with_capacity(n_cols),
            lower: Vec::with_capacity(n_cols),
            upper: Vec::with_capacity(n_cols),
            trim: (alpha as usize) / 2,
            len,
        }
    }

    fn data(&self) -> &PartialHistograms {
        &self.data
    }

    fn init(&mut self) {
        self.sums = vec![0; self.data.n_cols];
        self.lower = vec![Vec::with_capacity(self.trim); self.data.n_cols];
        self.upper = vec![Vec::with_capacity(self.trim); self.data.n_cols];
    }

    // By some miracle, this seems to work!
    fn update(&mut self, p_in: &[&[u8]], channel_index: usize, add: bool) {
        if !self.sums.is_empty() {
            if add {
                for n in 0..self.data.n_cols {
                    for i in n..(n + self.data.size) {
                        let val = p_in[i][channel_index];
                        let lower = self.lower(n);
                        let upper = self.upper(n);

                        if val < lower {
                            self.lower[n].remove(self.trim -  1);
                            self.sums[n] += lower as i32;

                            let pos = self.lower[n].binary_search(&val).unwrap_or_else(|e| e);
                            self.lower[n].insert(pos, val);
                        } else if val > upper {
                            self.upper[n].remove(self.trim - 1);
                            self.sums[n] += upper as i32;

                            let pos = self.lower[n].binary_search_by_key(&Reverse(&val), Reverse).unwrap_or_else(|e| e);
                            self.upper[n].insert(pos, val);
                        } else {
                            self.sums[n] += val as i32;
                        }
                    }
                }
                self.data.update(p_in, channel_index, add);
            } else {
                self.data.update(p_in, channel_index, add);
                for n in 0..self.data.n_cols {
                    for i in n..(n + self.data.size) {
                        let val = p_in[i][channel_index];
                        let lower = self.lower(n);
                        let upper = self.upper(n);

                        let mut lower_count = self.data.get_count(lower as usize, n);
                        let mut upper_count = self.data.get_count(upper as usize, n);

                        for j in i..(n + self.data.size) {
                            if p_in[j][channel_index] == lower {
                                lower_count += 1;
                            } else if p_in[j][channel_index] == upper {
                                upper_count += 1;
                            }
                        }

                        for j in self.lower[n].iter().rev() {
                            if *j == lower {
                                lower_count -= 1;
                            } else {
                                break;
                            }
                        }

                        for j in self.upper[n].iter().rev() {
                            if *j == upper {
                                upper_count -= 1;
                            } else {
                                break;
                            }
                        }

                        if val == lower && lower_count == 0 {
                            self.lower[n].remove(self.trim - 1);
                            self.get_next_lower(n, lower_count, lower);
                        } else if val < lower {
                            let res = self.lower[n].binary_search(&val);

                            match res {
                                Ok(pos) => {
                                    self.lower[n].remove(pos);
                                    self.get_next_lower(n, lower_count, lower);
                                },
                                Err(_) => {
                                    self.sums[n] -= val as i32;
                                }
                            }
                        } else if val == upper && upper_count == 0 {
                            self.upper[n].remove(self.trim - 1);
                            self.get_next_upper(n, upper_count, upper);
                        } else if val > upper {
                            let res = self.lower[n].binary_search_by_key(&Reverse(&val), Reverse);

                            match res {
                                Ok(pos) => {
                                    self.upper[n].remove(pos);
                                    self.get_next_upper(n, upper_count, upper);
                                },
                                Err(_) => {
                                    self.sums[n] -= val as i32;
                                }
                            }
                        } else {
                            self.sums[n] -= val as i32;
                        }
                    }
                }
            }
        } else {
            self.data.update(p_in, channel_index, add);
        }
    }

    fn set_sum(&mut self, sum: i32, index: usize) {
        self.sums[index] = sum;
    }

    fn set_upper(&mut self, vals: Vec<u8>, index: usize) {
        self.upper[index] = vals;
    }

    fn set_lower(&mut self, vals: Vec<u8>, index: usize) {
        self.lower[index] = vals;
    }

    fn upper(&self, index: usize) -> u8 {
        self.upper[index][self.trim-1]
    }

    fn lower(&self, index: usize) -> u8 {
        self.lower[index][self.trim-1]
    }

    fn get_mean(&self, index: usize) -> u8 {
        ((self.sums[index] as f32) / self.len).round() as u8
    }

    fn get_next_lower(&mut self, n: usize, lower_count: i32, lower: u8) {
        if lower_count > 0 {
            self.lower[n].push(lower);
            self.sums[n] -= lower as i32;
        } else {
            for key in (lower + 1)..=255 {
                if self.data.get_count(key as usize, n) > 0 {
                    self.lower[n].push(key);
                    self.sums[n] -= key as i32;
                    break;
                }
            }
        }
    }

    fn get_next_upper(&mut self, n: usize, upper_count: i32, upper: u8) {
        if upper_count > 0 {
            self.upper[n].push(upper);
            self.sums[n] -= upper as i32;
        } else {
            for key in (0..upper).rev() {
                if self.data.get_count(key as usize, n) > 0 {
                    self.upper[n].push(key);
                    self.sums[n] -= key as i32;
                    break;
                }
            }
        }
    }
}

fn process_cols_mean(input: &Image<u8>, output: &mut Image<u8>, radius: u32, alpha: u32, n_cols: usize, x: u32) {
    let (width, height, channels) = input.info().whc();
    let mut histograms = vec![MeanHist::new(radius as usize, n_cols, alpha); channels as usize];
    let mut p_out = Vec::with_capacity(channels as usize);

    // Initialize histogram and process first row
    init_cols_mean(input, output, &mut histograms, &mut p_out, radius, alpha, n_cols, x);

    // Update histogram and process remaining rows
    let mut row_in = Vec::with_capacity(n_cols);
    let mut row_out = Vec::with_capacity(n_cols);
    for j in 1..height {
        // Update histograms
        let j_in = (j + radius).clamp(0, input.info().height - 1);
        let j_out = (j as i32 - radius as i32 - 1).clamp(0, input.info().height as i32 - 1) as u32;

        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            let i_clamp = i.clamp(0, width as i32 - 1) as u32;
            row_in.push(input.get_pixel_unchecked(i_clamp, j_in));
            row_out.push(input.get_pixel_unchecked(i_clamp, j_out));
        }

        add_row_mean(&mut histograms, &row_in);
        remove_row_mean(&mut histograms, &row_out);

        process_row_mean(output, &mut histograms, &mut p_out, n_cols, x, j);

        row_in.clear();
        row_out.clear();
    }
}

fn init_cols_mean(input: &Image<u8>, output: &mut Image<u8>, histograms: &mut Vec<MeanHist>,
                  p_out: &mut Vec<u8>, radius: u32, alpha: u32, n_cols: usize, x: u32) {
    let (width, height) = input.info().wh();
    let size = 2 * radius + 1;

    // Initialize histograms
    let mut row_in = Vec::with_capacity(n_cols);
    for j in -(radius as i32)..(radius as i32 + 1) {
        for i in (x as i32 - radius as i32)..((x + n_cols as u32 + radius) as i32) {
            row_in.push(input.get_pixel_unchecked(i.clamp(0, width as i32 - 1) as u32,
                                                j.clamp(0, height as i32 - 1) as u32));
        }

        add_row_mean(histograms, &row_in);
    }

    // Initialize histograms
    for hist in histograms.iter_mut() {
        hist.init();
    }

    // Compute first mean values
    let trim = (alpha as usize) / 2;
    let upper_trim = (size * size) as usize - trim;
    for i in 0..n_cols {
        p_out.clear();
        for hist in histograms.iter_mut() {
            let mut count = 0;
            let mut sum = 0;
            let mut upper = Vec::with_capacity(trim);
            let mut lower = Vec::with_capacity(trim);

            for key in 0u8..=255 {
                let mut add = hist.data().get_count(key as usize, i);
                count += add;
                sum += add * key as i32;

                while lower.len() < trim && add > 0 {
                    lower.push(key);
                    sum -= key as i32;
                    add -= 1;
                }

                while (count as usize) > upper_trim && upper.len() < trim && add > 0 {
                    upper.insert(0, key);
                    sum -= key as i32;
                    add -= 1;
                }
            }

            hist.set_sum(sum, i);
            hist.set_upper(upper, i);
            hist.set_lower(lower, i);

            p_out.push(hist.get_mean(i));
        }

        let x_clamp = (x + i as u32).clamp(0, output.info().width - 1);
        output.set_pixel(x_clamp, 0, &p_out);
    }
}

fn process_row_mean(output: &mut Image<u8>, histograms: &mut Vec<MeanHist>, p_out: &mut Vec<u8>, n_cols: usize, x: u32, y: u32) {
    for i in 0..n_cols {
        p_out.clear();
        for hist in histograms.iter_mut() {
            p_out.push(hist.get_mean(i));
        }

        let x_clamp = (x + i as u32).clamp(0, output.info().width - 1);
        output.set_pixel(x_clamp, y, &p_out);
    }
}

fn add_row_mean(histograms: &mut Vec<MeanHist>, p_in: &[&[u8]]) {
    for (c, hist) in histograms.iter_mut().enumerate() {
        hist.update(p_in, c, true);
    }
}

fn remove_row_mean(histograms: &mut Vec<MeanHist>, p_in: &[&[u8]]) {
    for (c, hist) in histograms.iter_mut().enumerate() {
        hist.update(p_in, c, false);
    }
}