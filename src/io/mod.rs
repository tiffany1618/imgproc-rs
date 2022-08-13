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

use crate::error::{ImgIoError, ImgIoResult};
use crate::image::{Image, BaseImage};

use image::io::Reader;
use image::{GenericImageView, ColorType, ImageBuffer};

/// Extracts channels and alpha from an `image::ColorType`
fn from_color_type(color: ColorType) -> ImgIoResult<(u8, bool)> {
    match color {
        ColorType::L8 => Ok((1, false)),
        ColorType::La8 => Ok((2, true)),
        ColorType::Rgb8 => Ok((3, false)),
        ColorType::Rgba8 => Ok((4, true)),
        _ => Err(ImgIoError::UnsupportedColorTypeError("unsupported color type".to_string()))
    }
}

// TODO: Fix rotation of JPG images where width < height
/// Reads an image file into an `Image<u8>`. A wrapper around `image::io::Reader::open()`
pub fn read(filename: &str) -> ImgIoResult<Image<u8>> {
    let img = Reader::open(filename)?.decode()?;
    let (width, height) = img.dimensions();
    let (channels, alpha) = from_color_type(img.color())?;

    Ok(Image::from_slice(width, height, channels, alpha, img.as_bytes()))
}

/// Writes an RGB(A)8 or Gray(A)8 `Image<u8>` into an image file. A wrapper around `image::io::Reader::save()`
pub fn write(input: &Image<u8>, filename: &str) -> ImgIoResult<()> {
    let (width, height, channels, alpha) = input.info().whca();

    if alpha {
        match channels {
            2 => {
                let img_buf: ImageBuffer<image::LumaA<u8>, &[u8]> = ImageBuffer::from_raw(width, height, input.data())
                    .ok_or_else(|| ImgIoError::ImageWriteError("ImageBuffer Container is not big enough".to_string()))?;
                img_buf.save(filename)?;
            },
            4 => {
                let img_buf: ImageBuffer<image::Rgba<u8>, &[u8]> = ImageBuffer::from_raw(width, height, input.data())
                    .ok_or_else(|| ImgIoError::ImageWriteError("ImageBuffer Container is not big enough".to_string()))?;
                img_buf.save(filename)?;
            },
            _ => return Err(ImgIoError::UnsupportedColorTypeError("unsupported color type".to_string()))
        }
    } else {
        match channels {
            1 => {
                let img_buf: ImageBuffer<image::Luma<u8>, &[u8]> = ImageBuffer::from_raw(width, height, input.data())
                    .ok_or_else(|| ImgIoError::ImageWriteError("ImageBuffer Container is not big enough".to_string()))?;
                img_buf.save(filename)?;
            },
            3 => {
                let img_buf: ImageBuffer<image::Rgb<u8>, &[u8]> = ImageBuffer::from_raw(width, height, input.data())
                    .ok_or_else(|| ImgIoError::ImageWriteError("ImageBuffer Container is not big enough".to_string()))?;
                img_buf.save(filename)?;
            },
            _ => return Err(ImgIoError::UnsupportedColorTypeError("unsupported color type".to_string()))
        }
    }

    Ok(())
}