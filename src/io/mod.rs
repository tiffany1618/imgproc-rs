#![cfg(not(doctest))]
//! A module for image reading/writing
//!
//! # Examples
//! ```rust
//! # use imgproc_rs::error::ImgIoResult;
//! # use imgproc_rs::image::BaseImage;
//! #
//! # fn main() -> ImgIoResult<()> {
//! // Read an image from a path
//! let img = imgproc_rs::io::read("path/to/image.png")?;
//!
//! // Print the image information
//! println!("{}", img.info());
//!
//! // Write the image to a path as a PNG
//! imgproc_rs::io::write(&img, "path/to/save_image.png")?;
//! # Ok(())
//! # }
//! ```

pub use self::io::*;

mod io;