use crate::error::{ImgIoError, ImgIoResult};
use crate::image::{Image, BaseImage};

use image::io::Reader;
use image::{GenericImageView, ColorType, DynamicImage};
use rulinalg::matrix::Axes::Col;
use image::error::UnsupportedErrorKind::Color;

/// Extracts channels and alpha from an `image::ColorType`
fn from_color_type(color: ColorType) -> ImgIoResult<(u8, bool)> {
    match color {
        ColorType::L8 => Ok((1, false)),
        ColorType::La8 => Ok((2, true)),
        ColorType::Rgb8 | ColorType::Bgr8 => Ok((3, false)),
        ColorType::Rgba8 | ColorType::Bgra8 => Ok((4, true)),
        _ => Err(ImgIoError::UnsupportedColorType("unsupported color type".to_string()))
    }
}

/// Converts channels and alpha to a valid `image::ColorType`
fn into_color_type(channels: u8, alpha: bool) -> ImgIoResult<ColorType> {
    if alpha {
        match channels {
            2 => Ok(ColorType::La8),
            4 => Ok(ColorType::Rgba8),
            _ => Err(ImgIoError::UnsupportedColorType("unsupported color type".to_string()))
        }
    } else {
        match channels {
            1 => Ok(ColorType::L8),
            3 => Ok(ColorType::Rgb8),
            _ => Err(ImgIoError::UnsupportedColorType("unsupported color type".to_string()))
        }
    }
}

/// Reads an image file into an `Image<u8>`. A wrapper around `image::io::Reader::open()`
pub fn read(filename: &str) -> ImgIoResult<Image<u8>> {
    let img = Reader::open(filename)?.decode()?;
    let (width, height) = img.dimensions();
    let (channels, alpha) = from_color_type(img.color())?;

    Ok(Image::new(width, height, channels, alpha, img.as_bytes()))
}

/// Writes an RGB(A)8 or Gray(A)8 `Image<u8>` into an image file. A wrapper around `image::io::Reader::save()`
pub fn write(input: &Image<u8>, filename: &str) -> ImgIoResult<()> {
    let (width, height, channels, alpha) = input.info().whca();

    match into_color_type(channels, alpha)? {
        ColorType::L8 => Ok(DynamicImage::new_luma8(width, height).save(filename)?),
        ColorType::La8 => Ok(DynamicImage::new_luma_a8(width, height).save(filename)?),
        ColorType::Rgb8 => Ok(DynamicImage::new_rgb8(width, height).save(filename)?),
        ColorType::Rgba8 => Ok(DynamicImage::new_rgba8(width, height).save(filename)?),
        _ => Err(ImgIoError::UnsupportedColorType("unsupported color type".to_string()))
    }
}