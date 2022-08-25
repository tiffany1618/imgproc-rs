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

pub use self::from_impl::*;
pub use self::image::*;
pub use self::pixel::*;
pub use self::pixel_iter::*;
pub use self::sub_image::*;

mod from_impl;
mod image;
mod pixel;
mod pixel_iter;
mod sub_image;