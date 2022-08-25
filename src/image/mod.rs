//! A module for the core image structs and traits
//!
//! # Examples
//! ```rust
//! # use imgproc_rs::error::ImgIoResult;
//!
//! # fn main() {
//! use imgproc_rs::image::{Image, BaseImage, ImageInfo};
//!
//! let vec = vec![1, 2, 3, 4, 5, 6,
//!                7, 8, 9, 10, 11, 12];
//!
//! // Create an image from a slice
//! let img_slice = Image::from_slice(2, 2, 3, false, &vec);
//!
//! // Create an image from a vector
//! let img_vec = Image::from_vec(2, 2, 3, false, vec);
//!
//! // Create a blank (black) image
//! let mut img_blank: Image<u8> = Image::blank(ImageInfo::new(2, 2, 3, false));
//!
//! // Create an empty image
//! let mut img_empty: Image<u8> = Image::empty(ImageInfo::new(2, 2, 3, false));
//!
//! // Get width and height of image
//! let (width, height) = img_slice.info().wh();
//!
//! // Get width, height, and channels of image
//! let (width, height, channels) = img_slice.info().whc();
//!
//! // Get width, height, channels, and alpha of image
//! let (width, height, channels, alpha) = img_slice.info().whca();
//!
//! // Set and get an image pixel using a 1D index (reads the image data row by row from left to
//! // right, starting in the upper left corner of the image)
//! img_blank.set_pixel_indexed(0, &[1, 1, 1]);
//! let pixel_1d = &img_blank[0];
//!
//! // Set and get an image pixel using 2D coordinates (coordinates start at zero in the upper
//! // left corner of the image and increase downwards and to the right)
//! img_blank.set_pixel(1, 1, &[1, 1, 1]);
//! let pixel_2d = img_vec.get_pixel(1, 1);
//!
//! /* Print image information
//!  * Example output:
//!  *
//!  * width: 2
//!  * height: 2
//!  * channels: 3
//!  * alpha: false
//!  *
//!  */
//! println!("{}", img_slice.info());
//!
//! # }
//! ```

pub use self::sub_image::*;
pub use self::pixel::*;
pub use self::from_impl::*;
pub use self::pixel_iter::*;

mod sub_image;
mod pixel;
mod from_impl;
mod pixel_iter;

use num_traits::NumAssign;

use crate::error;

/// A struct representing an image
#[derive(Debug, Clone, PartialEq)]
pub struct Image<T: Number> {
    info: ImageInfo,
    data: Vec<T>,
}

/// A struct containing image information
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub alpha: bool,
}

/// A trait for valid image channel types
pub trait Number:
NumAssign
+ std::marker::Copy
+ std::fmt::Display
+ std::cmp::PartialOrd
+ std::marker::Sync
+ std::marker::Send
+ From<u8>
    where Self: std::marker::Sized {}

impl<T> Number for T
    where T:
    NumAssign
    + std::marker::Copy
    + std::fmt::Display
    + std::cmp::PartialOrd
    + std::marker::Sync
    + std::marker::Send
    + From<u8> {}

/// A trait for a base image
pub trait BaseImage<T: Number> {
    /// Returns the image information
    fn info(&self) -> ImageInfo;

    /// Returns a slice representing the pixel located at `(x, y)`
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is out of bounds
    fn get_pixel(&self, x: u32, y: u32) -> &[T];
}

impl ImageInfo {
    /// Creates a new ImageInfo
    pub fn new(width: u32, height: u32, channels: u8, alpha: bool) -> Self {
        ImageInfo { width, height, channels, alpha }
    }

    /// Returns the width and height of the image
    pub fn wh(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the width, height, and channels of the image
    pub fn whc(&self) -> (u32, u32, u8) {
        (self.width, self.height, self.channels)
    }

    /// Returns the width, height, channels, and alpha of the image
    pub fn whca(&self) -> (u32, u32, u8, bool) {
        (self.width, self.height, self.channels, self.alpha)
    }

    /// Returns the number of non alpha channels in the image
    pub fn channels_non_alpha(&self) -> u8 {
        if self.alpha {
            self.channels - 1
        } else {
            self.channels
        }
    }

    /// Returns the size of the image (width * height)
    pub fn size(&self) -> u32 {
        self.width * self.height
    }

    /// Returns the full size of the image (width * height * channels)
    pub fn full_size(&self) -> u32 {
        self.width * self.height * (self.channels as u32)
    }
}

impl std::fmt::Display for ImageInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {}\nheight: {}\nchannels: {}\nalpha: {}", self.width, self.height, self.channels, self.alpha)
    }
}

impl<T: Number> Image<T> {
    /// Creates a new `Image<T>` from a slice
    pub fn from_slice(width: u32, height: u32, channels: u8, alpha: bool, data: &[T]) -> Self {
        Image {
            info: ImageInfo{ width, height, channels, alpha },
            data: data.to_vec(),
        }
    }

    /// Creates a new `Image<T>` from a vector
    pub fn from_vec(width: u32, height: u32, channels: u8, alpha: bool, data: Vec<T>) -> Self {
        Image {
            info: ImageInfo{ width, height, channels, alpha },
            data,
        }
    }

    /// Creates a new `Image<T>` from a vector of vectors
    pub fn from_vec_of_vec(width: u32, height: u32, channels: u8, alpha: bool, data: Vec<Vec<T>>) -> Self {
        let mut data_vec = Vec::with_capacity((width * height * channels as u32) as usize);
        for vec in &data {
            data_vec.extend_from_slice(vec)
        }

        Image {
            info: ImageInfo{ width, height, channels, alpha },
            data: data_vec,
        }
    }

    /// Creates a new `Image<T>` from a vector of slices
    pub fn from_vec_of_slice(width: u32, height: u32, channels: u8, alpha: bool, data: Vec<&[T]>) -> Self {
        let mut data_vec = Vec::with_capacity((width * height * channels as u32) as usize);
        for vec in data {
            data_vec.extend_from_slice(vec)
        }

        Image {
            info: ImageInfo{ width, height, channels, alpha },
            data: data_vec,
        }
    }

    /// Creates an `Image<T>` populated with zeroes
    pub fn blank(info: ImageInfo) -> Self {
        Image {
            info,
            data: vec![0.into(); info.full_size() as usize],
        }
    }

    /// Creates an empty `Image<T>`
    pub fn empty(info: ImageInfo) -> Self {
        Image {
            info,
            data: Vec::with_capacity(info.full_size() as usize),
        }
    }

    /// Returns the 1d index corresponding to the 2d `(x, y)` indices
    pub fn index(&self, x: u32, y: u32) -> usize {
        ((y * self.info.width + x) * self.info.channels as u32) as usize
    }

    /// Returns all data as a slice
    pub fn data(&self) -> &[T] {
        &self.data[..]
    }

    /// Returns all data as a mutable slice
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data[..]
    }

    /// Returns a slice representing the pixel located at `(x, y)` without checking index bounds
    pub fn get_pixel_unchecked(&self, x: u32, y: u32) -> &[T] {
        &self[(y * self.info.width + x) as usize]
    }

    /// Returns a mutable slice representing the pixel located at `(x, y)`
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is out of bounds
    pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut [T] {
        error::check_xy(x, y, self.info.width, self.info.height);

        let start = self.index(x, y);
        &mut self.data[start..(start + self.info.channels as usize)]
    }

    /// Returns a mutable slice representing the pixel located at `(x, y)` without checking index
    /// bounds
    pub fn get_pixel_mut_unchecked(&mut self, x: u32, y: u32) -> &mut [T] {
        let index = (y * self.info.width + x) as usize;
        &mut self[index]
    }

    /// Returns a `SubImage<T>` representing the part of the image of width `width` and height
    /// `height`, with upper left hand corner located at `(x, y)`
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is out of bounds
    pub fn get_subimage(&self, x: u32, y: u32, width: u32, height: u32) -> SubImage<T> {
        error::check_xy(x, y, self.info.width, self.info.height);

        let mut data = Vec::new();

        for i in x..(x + width) {
            for j in y..(y + height) {
                data.push(self.get_pixel(i, j));
            }
        }

        SubImage::new(width, height, self.info.channels, self.info.alpha, data)
    }

    /// Returns a `SubImage<T>` representing the row or column of pixels of length `size` centered at
    /// `(x, y)`. If `is_vert` is `true`, returns the column; otherwise, returns the row.
    /// Uses clamp padding for edge pixels (edge pixels are repeated indefinitely)
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is out of bounds
    pub fn get_neighborhood_1d(&self, x: u32, y: u32, size: u32, is_vert: bool) -> SubImage<T> {
        error::check_xy(x, y, self.info.width, self.info.height);

        let mut data = Vec::new();

        if is_vert {
            let start_y = (y as i32) - (size as i32) / 2;

            for i in 0..size {
                let mut curr_y = start_y + (i as i32);
                if curr_y < 0 || curr_y >= self.info.height as i32 { curr_y = y as i32 };

                data.push(self.get_pixel(x, curr_y as u32));
            }

            SubImage::new(1, size, self.info.channels, self.info.alpha, data)
        } else {
            let start_x = (x as i32) - (size as i32) / 2;

            for i in 0..size {
                let mut curr_x = start_x + (i as i32);
                if curr_x < 0 || curr_x >= self.info.width as i32 { curr_x = x as i32 };

                data.push(self.get_pixel(curr_x as u32, y));
            }

            SubImage::new(size, 1, self.info.channels, self.info.alpha, data)
        }
    }

    /// Returns a `SubImage<T>` representing the "square" of pixels of side length `size` centered
    /// at `(x, y)`. Uses clamp padding for edge pixels (edge pixels are repeated indefinitely)
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is out of bounds
    pub fn get_neighborhood_2d(&self, x: u32, y: u32, size: u32) -> SubImage<T> {
        error::check_xy(x, y, self.info.width, self.info.height);

        let start_x = (x as i32) - (size as i32) / 2;
        let start_y = (y as i32) - (size as i32) / 2;

        let mut data = Vec::new();
        for i in 0..size {
            for j in 0..size {
                let mut curr_x = start_x + (j as i32);
                let mut curr_y = start_y + (i as i32);

                if curr_x < 0 || curr_x >= self.info.width as i32 { curr_x = x as i32 };
                if curr_y < 0 || curr_y >= self.info.height as i32 { curr_y = y as i32 };

                data.push(self.get_pixel(curr_x as u32, curr_y as u32));
            }
        }

        SubImage::new(size, size, self.info.channels, self.info.alpha, data)
    }

    /// Replaces the pixel located at `(x, y)` with `pixel`
    ///
    /// # Panics
    ///
    /// Panics if the length of `pixel` is not equal to the number of channels in the image or `x`
    /// or `y` is out of bounds
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: &[T]) {
        error::check_channels(self.info.channels, pixel.len());
        error::check_xy(x, y, self.info.width, self.info.height);

        let start = self.index(x, y);
        self.data[start..((self.info.channels as usize) + start)].clone_from_slice(pixel);
    }

    /// Replaces the pixel at index `index` with `pixel`
    ///
    /// # Panics
    ///
    /// Panics if the length of `pixel` is not equal to the number of channels in the image
    pub fn set_pixel_indexed(&mut self, index: usize, pixel: &[T]) {
        error::check_channels(self.info.channels, pixel.len());

        let start = index * self.info.channels as usize;
        self.data[start..((self.info.channels as usize) + start)].clone_from_slice(pixel);
    }

    /// Applies function `f` to each pixel
    pub fn map_pixels<S: Number, F>(&self, f: F) -> Image<S>
        where F: Fn(&[T], &mut Vec<S>) {
        let mut data= Vec::with_capacity(self.info.size() as usize);
        let mut p_out = Vec::new();

        for i in 0..(self.info.size() as usize) {
            f(&self[i], &mut p_out);
            data.append(&mut p_out);
        }

        let channels = (data.len() as u32 / self.info.size()) as u8;

        Image {
            info: ImageInfo {
                width: self.info.width,
                height: self.info.height,
                channels,
                alpha: self.info.alpha
            },
            data,
        }
    }

    /// If `alpha`, applies function `f` to the non-alpha portion of each pixel and applies
    /// function `g` to the alpha channel of each pixel; otherwise, applies function `f` to
    /// each pixel
    pub fn map_pixels_if_alpha<S: Number, F, G>(&self, f: F, g: G) -> Image<S>
        where F: Fn(&[T], &mut Vec<S>),
              G: Fn(T) -> S {
        if !self.info.alpha {
            return self.map_pixels(f);
        }

        let mut data = Vec::with_capacity(self.info.size() as usize);
        let mut p_out = Vec::new();

        for i in 0..(self.info.size() as usize) {
            f(self[i].channels_without_alpha(), &mut p_out);
            p_out.push(g(self[i].alpha()));
            data.append(&mut p_out);
        }

        let channels = (data.len() as u32 / self.info.size()) as u8;

        Image {
            info: ImageInfo {
                width: self.info.width,
                height: self.info.height,
                channels,
                alpha: self.info.alpha
            },
            data,
        }
    }

    /// Applies function `f` to each channel of each pixel
    pub fn map_channels<S: Number, F>(&self, f: F) -> Image<S>
        where F: Fn(T) -> S {
        let mut data = Vec::with_capacity(self.info.full_size() as usize);

        for i in 0..(self.info.full_size() as usize) {
            data.push(f(self.data[i]));
        }

        Image {
            info: self.info,
            data,
        }
    }

    /// If `alpha`, applies function `f` to each non-alpha channel of each pixel and
    /// applies function `g` to the alpha channel of each pixel;
    /// otherwise, applies function `f` to each channel of each pixel
    pub fn map_channels_if_alpha<S: Number, F, G>(&self, f: F, g: G) -> Image<S>
        where F: Fn(T) -> S,
              G: Fn(T) -> S {
        if !self.info.alpha {
            return self.map_channels(f);
        }

        let mut data = Vec::with_capacity(self.info.full_size() as usize);
        for i in 0..(self.info.size() as usize) {
            data.append(&mut self[i].map_alpha(&f, &g));
        }

        Image {
            info: self.info,
            data,
        }
    }

    /// Applies function `f` to each pixel
    pub fn apply_pixels<F>(&mut self, f: F)
        where F: Fn(&[T], &mut Vec<T>) {
        let mut p_out = Vec::with_capacity(self.info.channels as usize);

        for i in 0..self.info.size() as usize {
            p_out.clear();
            f(&self[i], &mut p_out);
            self.set_pixel_indexed(i, &p_out);
        }
    }

    /// If `alpha`, applies function `f` to the non-alpha portion of each pixel and
    /// applies function `g` to the alpha channel of each pixel;
    /// otherwise, applies function `f` to each pixel
    pub fn apply_pixels_if_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(&[T], &mut Vec<T>),
              G: Fn(T) -> T {
        if !self.info.alpha {
            self.apply_pixels(f);
            return;
        }

        let mut p_out = Vec::with_capacity(self.info.channels as usize);
        for i in 0..(self.info.size() as usize) {
            p_out.clear();
            f(self[i].channels_without_alpha(), &mut p_out);
            p_out.push(g(self[i].alpha()));
            self.set_pixel_indexed(i, p_out.as_slice());
        }
    }

    /// Applies function `f` to each channel of each pixel
    pub fn apply_channels<F>(&mut self, f: F)
        where F: Fn(T) -> T {
        for i in 0..(self.info.full_size() as usize) {
            self.data[i] = f(self.data[i]);
        }
    }

    /// If `alpha`, applies function `f` to each non-alpha channel of each pixel and
    /// applies function `g` to the alpha channel of each pixel;
    /// otherwise, applies function `f` to each channel of each pixel
    pub fn apply_channels_if_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(T) -> T,
              G: Fn(T) -> T {
        if !self.info.alpha {
            self.apply_channels(f);
            return;
        }

        for i in 0..(self.info.size() as usize) {
            self[i].apply_alpha(&f, &g);
        }
    }

    /// Applies function `f` to each channel of index `index` of each pixel. Modifies `self`
    pub fn edit_channel<F>(&mut self, f: F, index: usize)
        where F: Fn(T) -> T {
        for i in (index..(self.info.full_size() as usize)).step_by(self.info.channels as usize) {
            self.data[i] = f(self.data[i]);
        }
    }
}

impl<T: Number> BaseImage<T> for Image<T> {
    fn info(&self) -> ImageInfo {
        self.info
    }

    fn get_pixel(&self, x: u32, y: u32) -> &[T] {
        error::check_xy(x, y, self.info.width, self.info.height);

        &self[(y * self.info.width + x) as usize]
    }
}

impl<T: Number> std::ops::Index<usize> for Image<T> {
    type Output = [T];

    fn index(&self, i: usize) -> &Self::Output {
        let start = i * (self.info.channels as usize);
        &self.data[start..(start + self.info.channels as usize)]
    }
}

impl<T: Number> std::ops::IndexMut<usize> for Image<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        let start = i * (self.info.channels as usize);
        &mut self.data[start..(start + self.info.channels as usize)]
    }
}