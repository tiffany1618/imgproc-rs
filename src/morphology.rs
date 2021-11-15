//! A module for image morphology operations

use crate::{error, util};
use crate::error::ImgProcResult;
use crate::image::{Image, BaseImage};

/// Erodes a binary image (grayscale image with pixel values of 0 or 255) using a kernel of size
/// `(2 * radius + 1) x (2 * radius + 1)`
pub fn erode(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    error::check_grayscale(input)?;

    let (width, height) =  input.info().wh();
    let size = 2 * radius + 1;
    let max_sum = (size * size * 255) as f32;
    let table = util::generate_summed_area_table(&input.clone().into());
    let mut output = Image::blank(input.info());

    for y in 0..height {
        for x in 0..width {
            let mut x_top = x;
            let mut x_bot = x;
            let mut y_top = y;
            let mut y_bot = y;

            if x >= radius {
                x_top -= radius;
            }
            if x < width - radius {
                x_bot += radius;
            }
            if y >= radius {
                y_top -= radius;
            }
            if y < height - radius {
                y_bot += radius;
            }

            if (util::rectangular_intensity_sum(&table, x_top, y_top, x_bot, y_bot)[0] - max_sum).abs() < f32::EPSILON {
                output.set_pixel(x, y, &[255]);
            }
        }
    }

    Ok(output)
}

/// Dilates a binary image (grayscale image with pixel values of 0 or 255) using a kernel of size
/// `(2 * radius + 1) x (2 * radius + 1)`
pub fn dilate(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    error::check_grayscale(input)?;

    let (width, height) =  input.info().wh();
    let table = util::generate_summed_area_table(&input.clone().into());
    let mut output = Image::blank(input.info());

    for y in 0..height {
        for x in 0..width {
            let mut x_top = x;
            let mut x_bot = x;
            let mut y_top = y;
            let mut y_bot = y;

            if x >= radius {
                x_top -= radius;
            }
            if x < width - radius {
                x_bot += radius;
            }
            if y >= radius {
                y_top -= radius;
            }
            if y < height - radius {
                y_bot += radius;
            }

            if util::rectangular_intensity_sum(&table, x_top, y_top, x_bot, y_bot)[0] >= 255.0 {
                output.set_pixel(x, y, &[255]);
            }
        }
    }

    Ok(output)
}

/// Sets output pixel to the majority-valued pixel in the input image under a kernel of size
/// `(2 * radius + 1) x (2 * radius + 1)`
pub fn majority(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    error::check_grayscale(input)?;

    let (width, height) =  input.info().wh();
    let table = util::generate_summed_area_table(&input.clone().into());
    let mut output = Image::blank(input.info());

    for y in 0..height {
        for x in 0..width {
            let mut x_top = x;
            let mut x_bot = x;
            let mut y_top = y;
            let mut y_bot = y;

            if x >= radius {
                x_top -= radius;
            }
            if x < width - radius {
                x_bot += radius;
            }
            if y >= radius {
                y_top -= radius;
            }
            if y < height - radius {
                y_bot += radius;
            }

            if util::rectangular_intensity_sum(&table, x_top, y_top, x_bot, y_bot)[0] >= 255.0 {
                output.set_pixel(x, y, &[255]);
            }
        }
    }

    Ok(output)
}

/// Applies an erosion followed by a dilation
pub fn open(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    Ok(dilate(&erode(input, radius)?, radius)?)
}

/// Applies a dilation followed by an erosion
pub fn close(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    Ok(erode(&dilate(input, radius)?, radius)?)
}

/// Returns the difference between dilation and erosion of the image
#[allow(unused_parens)]
pub fn gradient(input: &Image<u8>, radius: u32) -> ImgProcResult<Image<u8>> {
    error::check_grayscale(input)?;

    let (width, height) =  input.info().wh();
    let size = 2 * radius + 1;
    let max_sum = (size * size * 255) as f32;
    let table = util::generate_summed_area_table(&input.clone().into());
    let mut output = Image::blank(input.info());

    for y in 0..height {
        for x in 0..width {
            let mut x_top = x;
            let mut x_bot = x;
            let mut y_top = y;
            let mut y_bot = y;

            if x >= radius {
                x_top -= radius;
            }
            if x < width - radius {
                x_bot += radius;
            }
            if y >= radius {
                y_top -= radius;
            }
            if y < height - radius {
                y_bot += radius;
            }

            let sum = util::rectangular_intensity_sum(&table, x_top, y_top, x_bot, y_bot)[0];
            let erode = ((sum - max_sum).abs() < f32::EPSILON);
            let dilate = (sum >= 255.0);

            if erode ^ dilate {
                output.set_pixel(x, y, &[255]);
            }
        }
    }

    Ok(output)
}