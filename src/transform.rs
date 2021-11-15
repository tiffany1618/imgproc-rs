//! A module for image transformation operations

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::enums::{Refl, Scale};
use crate::error;
use crate::error::{ImgProcError, ImgProcResult};
use crate::image::{BaseImage, Image, ImageInfo, Number};
use crate::util;

/// Crops an image to a rectangle with upper left corner located at `(x, y)` with width `width`
/// and height `height`
#[cfg(not(feature = "rayon"))]
pub fn crop<T: Number>(input: &Image<T>, x: u32, y: u32, width: u32, height: u32) -> ImgProcResult<Image<T>> {
    if (x + width) >= input.info().width {
        return Err(ImgProcError::InvalidArgError(format!("invalid width: input width is {} \
            but x + width is {}", input.info().width, (x + width))));
    } else if (y + height) >= input.info().height {
        return Err(ImgProcError::InvalidArgError(format!("invalid height: input height is {} \
            but y + height is {}", input.info().height, (y + height))));
    }

    let mut output = Image::blank(ImageInfo::new(width, height,
                                                 input.info().channels, input.info().alpha));

    for j in 0..height {
        for i in 0..width {
            output.set_pixel(i, j, input.get_pixel(i + x, j + y));
        }
    }

    Ok(output)
}

/// Crops an image to a rectangle with upper left corner located at `(x, y)` with width `width`
/// and height `height`
#[cfg(feature = "rayon")]
pub fn crop<T: Number>(input: &Image<T>, x: u32, y: u32, width: u32, height: u32) -> ImgProcResult<Image<T>> {
    if (x + width) >= input.info().width {
        return Err(ImgProcError::InvalidArgError(format!("invalid width: input width is {} \
            but x + width is {}", input.info().width, (x + width))));
    } else if (y + height) >= input.info().height {
        return Err(ImgProcError::InvalidArgError(format!("invalid height: input height is {} \
            but y + height is {}", input.info().height, (y + height))));
    }

    let size = width * height;
    let data: Vec<&[T]> = (0..size)
        .into_par_iter()
        .map(|i| {
            let (a, b) = util::get_2d_coords(i, width);
            input.get_pixel(a + x, b + y)
        })
        .collect();

    Ok(Image::from_vec_of_slice(width, height, input.info().channels, input.info().alpha, data))
}

/// Aligns the top left corner of `front` onto the location `(x, y)` on `back` and superimposes
/// the two images with weight `alpha` for pixel values of `back` and weight 1 - `alpha` for
/// pixel values of `front`
pub fn superimpose(back: &Image<f32>, front: &Image<f32>, x: u32, y: u32, alpha: f32) -> ImgProcResult<Image<f32>> {
    error::check_equal(back.info().channels, front.info().channels, "image channels")?;
    error::check_in_range(0.0, 1.0, alpha, "alpha")?;

    let mut output = back.clone();
    let width = std::cmp::min(x + front.info().width, back.info().width);
    let height = std::cmp::min(y + front.info().height, back.info().height);

    for j in y..height {
        for i in x..width {
            let mut p_in = Vec::with_capacity(output.info().channels as usize);
            let p_back = back.get_pixel(i, j);
            let p_front = front.get_pixel(i - x, j - y);
            for k in 0..(output.info().channels as usize) {
                p_in.push(alpha * p_back[k] + (1.0 - alpha) * p_front[k]);
            }

            output.set_pixel(i, j, &p_in);
        }
    }

    Ok(output)
}

/// Aligns the top left corner of `front` onto the location `(x, y)` on `back` and overlays
/// `front` on `back`
pub fn overlay<T: Number>(back: &Image<T>, front: &Image<T>, x: u32, y: u32) -> ImgProcResult<Image<T>> {
    error::check_equal(back.info().channels, front.info().channels, "image channels")?;

    let mut output = back.clone();
    let width = std::cmp::min(x + front.info().width, back.info().width);
    let height = std::cmp::min(y + front.info().height, back.info().height);

    for j in y..height {
        for i in x..width {
            output.set_pixel(i, j, front.get_pixel(i - x, j - y));
        }
    }

    Ok(output)
}

////////////////////////////
// Affine transformations
////////////////////////////

/// Scales an image horizontally by `x_factor` and vertically by `y_factor` using the specified
/// `method`
#[cfg(not(feature = "rayon"))]
pub fn scale(input: &Image<f32>, x_factor: f32, y_factor: f32, method: Scale) -> ImgProcResult<Image<f32>> {
    error::check_non_neg(x_factor, "x_factor")?;
    error::check_non_neg(y_factor, "y_factor")?;

    let width = (input.info().width as f32 * x_factor).round() as u32;
    let height = (input.info().height as f32 * y_factor).round() as u32;
    let mut output = Image::blank(ImageInfo::new(width, height,
                                                 input.info().channels, input.info().alpha));

    match method {
        Scale::NearestNeighbor => {
            scale_nearest_neighbor(input, &mut output, x_factor, y_factor);
        },
        Scale::Bilinear => {
            scale_bilinear(input, &mut output, x_factor, y_factor);
        },
        Scale::Bicubic => {
            scale_bicubic(input, &mut output, x_factor, y_factor);
        },
        Scale::Lanczos => {
            scale_lanczos_resampling(input, &mut output, x_factor, y_factor, 3);
        }
    }

    Ok(output)
}
/// Scales an image horizontally by `x_factor` and vertically by `y_factor` using the specified
/// `method`
#[cfg(feature = "rayon")]
pub fn scale(input: &Image<f32>, x_factor: f32, y_factor: f32, method: Scale) -> ImgProcResult<Image<f32>> {
    error::check_non_neg(x_factor, "x_factor")?;
    error::check_non_neg(y_factor, "y_factor")?;

    let width = (input.info().width as f32 * x_factor).round() as u32;
    let height = (input.info().height as f32 * y_factor).round() as u32;
    let info = ImageInfo::new(width, height, input.info().channels, input.info().alpha);

    return match method {
        Scale::NearestNeighbor => {
            Ok(scale_nearest_neighbor(input, &info, x_factor, y_factor))
        },
        Scale::Bilinear => {
            Ok(scale_bilinear(input, &info, x_factor, y_factor))
        },
        Scale::Bicubic => {
            Ok(scale_bicubic(input, &info, x_factor, y_factor))
        },
        Scale::Lanczos => {
            Ok(scale_lanczos_resampling(input, &info, x_factor, y_factor, 3))
        }
    }
}

/// Scales an image using Lanczos resampling with kernel of variable size `size`
#[cfg(not(feature = "rayon"))]
pub fn scale_lanczos(input: &Image<f32>, x_factor: f32, y_factor: f32, size: u32) -> ImgProcResult<Image<f32>> {
    error::check_non_neg(x_factor, "x_factor")?;
    error::check_non_neg(y_factor, "y_factor")?;
    error::check_non_neg(size, "size")?;

    let width = (input.info().width as f32 * x_factor).round() as u32;
    let height = (input.info().height as f32 * y_factor).round() as u32;
    let mut output = Image::blank(ImageInfo::new(width, height,
                                                 input.info().channels, input.info().alpha));

    scale_lanczos_resampling(input, &mut output, x_factor, y_factor, size);
    Ok(output)
}

/// Scales an image using Lanczos resampling with kernel of variable size `size`
#[cfg(feature = "rayon")]
pub fn scale_lanczos(input: &Image<f32>, x_factor: f32, y_factor: f32, size: u32) -> ImgProcResult<Image<f32>> {
    error::check_non_neg(x_factor, "x_factor")?;
    error::check_non_neg(y_factor, "y_factor")?;
    error::check_non_neg(size, "size")?;

    let width = (input.info().width as f32 * x_factor).round() as u32;
    let height = (input.info().height as f32 * y_factor).round() as u32;
    let info = ImageInfo::new(width, height, input.info().channels, input.info().alpha);

    Ok(scale_lanczos_resampling(input, &info, x_factor, y_factor, size))
}

/// Translates an image to the position with upper left corner located at `(x, y)`. Fills in the
/// rest of the image as black
pub fn translate<T: Number>(input: &Image<T>, x: u32, y: u32) -> ImgProcResult<Image<T>> {
    let mut output = Image::blank(input.info());

    for j in y..output.info().height {
        for i in x..output.info().width {
            output.set_pixel(i, j, input.get_pixel(i - x, j - y));
        }
    }

    Ok(output)
}

/// Rotates an image `degrees` degrees counterclockwise around the center of the image
pub fn rotate(input: &Image<f32>, degrees: f32) -> ImgProcResult<Image<f32>> {
    let (w_in, h_in) = input.info().wh();
    let (sin, cos) = degrees.to_radians().sin_cos();

    // Center coordinates
    let x = w_in / 2;
    let y = h_in / 2;

    let mat = [cos, -sin, sin, cos];

    // Compute dimensions of output image
    let coords1 = util::vector_mul(&mat, &[-(x as f32), y as f32])?;
    let coords2 = util::vector_mul(&mat, &[(w_in - x) as f32, y as f32])?;
    let coords3 = util::vector_mul(&mat, &[-(x as f32), (y as f32) - (h_in as f32)])?;
    let coords4 = util::vector_mul(&mat, &[(w_in - x) as f32, (y as f32) - (h_in as f32)])?;

    let x_max = util::max_4(coords1[0], coords2[0], coords3[0], coords4[0]);
    let x_min = util::min_4(coords1[0], coords2[0], coords3[0], coords4[0]);
    let y_max = util::max_4(coords1[1], coords2[1], coords3[1], coords4[1]);
    let y_min = util::min_4(coords1[1], coords2[1], coords3[1], coords4[1]);

    let w_out = (x_max - x_min) as u32;
    let h_out = (y_max - y_min) as u32;

    let mut output = Image::blank(ImageInfo::new(w_out, h_out,
                                                 input.info().channels, input.info().alpha));
    let mut coords = Vec::with_capacity(input.info().channels as usize);

    for j in 0..h_in {
        for i in 0..w_in {
            let x1 = (i as f32) - (x as f32);
            let y1 = (y as f32) - (j as f32);

            util::vector_mul_mut(&mat, &[x1, y1], &mut coords)?;

            coords[0] += x_max - 1.0;
            coords[1] = y_max - coords[1] - 1.0;

            output.set_pixel(coords[0] as u32, coords[1] as u32,
                             input.get_pixel(i, j));

            // Cover up "holes" left by rotation algorithm
            if coords[0] >= 1.0 {
                output.set_pixel(coords[0] as u32 - 1, coords[1] as u32, input.get_pixel(i, j));
            }
        }
    }

    Ok(output)
}

/// Reflects an image across the specified axis
pub fn reflect<T: Number>(input: &Image<T>, axis: Refl) -> ImgProcResult<Image<T>> {
    let mut output = Image::blank(input.info());
    let (width, height) = output.info().wh();

    match axis {
        Refl::Horizontal => {
            for y in 0..height {
                for x in 0..width {
                    output.set_pixel(x, y, input.get_pixel(x, height - y - 1));
                }
            }
        },
        Refl::Vertical => {
            for y in 0..height {
                for x in 0..width {
                    output.set_pixel(x, y, input.get_pixel(width - x - 1, y));
                }
            }
        },
    }

    Ok(output)
}

/// Shears an image
pub fn shear(input: &Image<f32>, shear_x: f32, shear_y: f32) -> ImgProcResult<Image<f32>> {
    let (w_in, h_in) = input.info().wh();
    let offset_x = (h_in as f32 * shear_x).abs();
    let offset_y = (w_in as f32 * shear_y).abs();
    let w_out = w_in + offset_x as u32;
    let h_out = offset_y as u32 + h_in;
    let mut output = Image::blank(ImageInfo::new(w_out, h_out,
                                                 input.info().channels, input.info().alpha));
    let mut coords = Vec::with_capacity(input.info().channels as usize);

    // Negative sign to give the conventional orientation for a positive shear, since the image
    // coordinates are flipped from conventional coordinates (i.e. (0,0) is in the top left corner
    // instead of the bottom left corner)
    let mat = [1.0, -shear_x, -shear_y, 1.0];

    for y in 0..h_in {
        for x in 0..w_in {
            util::vector_mul_mut(&mat, &[x as f32, y as f32], &mut coords)?;

            if shear_x > 0.0 {
                coords[0] += offset_x;
            }
            if shear_y > 0.0 {
                coords[1] += offset_y;
            }

            output.set_pixel(coords[0] as u32, coords[1] as u32, input.get_pixel(x, y));
        }
    }

    Ok(output)
}

///////////////////////
// Scaling Algorithms
///////////////////////

#[cfg(not(feature = "rayon"))]
fn scale_nearest_neighbor(input: &Image<f32>, output: &mut Image<f32>, x_factor: f32, y_factor: f32) {
    for y in 0..output.info().height {
        for x in 0..output.info().width {
            let p_out = interpolate_nearest_neighbor(input, x_factor, y_factor, x, y);
            output.set_pixel(x, y, p_out);
        }
    }
}

#[cfg(feature = "rayon")]
fn scale_nearest_neighbor(input: &Image<f32>, info: &ImageInfo, x_factor: f32, y_factor: f32) -> Image<f32> {
    let size = info.size();
    let (width, height, channels) = info.whc();

    let data: Vec<&[f32]> = (0..size)
        .into_par_iter()
        .map(|i| {
            let (x, y) = util::get_2d_coords(i, width);
            interpolate_nearest_neighbor(input, x_factor, y_factor, x, y)
        })
        .collect();

    Image::from_vec_of_slice(width, height, channels, info.alpha, data)
}

#[cfg(not(feature = "rayon"))]
fn scale_bilinear(input: &Image<f32>, output: &mut Image<f32>, x_factor: f32, y_factor: f32) {
    for y in 0..output.info().height {
        for x in 0..output.info().width {
            let p_out = interpolate_bilinear(input, x_factor, y_factor, x, y);
            output.set_pixel(x, y, &p_out);
        }
    }
}

#[cfg(feature = "rayon")]
fn scale_bilinear(input: &Image<f32>, info: &ImageInfo, x_factor: f32, y_factor: f32) -> Image<f32> {
    let size = info.size();
    let (width, height, channels) = info.whc();

    let data: Vec<Vec<f32>> = (0..size)
        .into_par_iter()
        .map(|i| {
            let (x, y) = util::get_2d_coords(i, width);
            interpolate_bilinear(input, x_factor, y_factor, x, y)
        })
        .collect();

    Image::from_vec_of_vec(width, height, channels, info.alpha, data)
}

#[cfg(not(feature = "rayon"))]
fn scale_bicubic(input: &Image<f32>, output: &mut Image<f32>, x_factor: f32, y_factor: f32) {
    for y in 0..output.info().height {
        for x in 0..output.info().width {
            let p_out = interpolate_bicubic(input, x_factor, y_factor, x, y);
            output.set_pixel(x, y, &p_out);
        }
    }
}

#[cfg(feature = "rayon")]
fn scale_bicubic(input: &Image<f32>, info: &ImageInfo, x_factor: f32, y_factor: f32) -> Image<f32> {
    let size = info.size();
    let (width, height, channels) = info.whc();

    let data: Vec<Vec<f32>> = (0..size)
        .into_par_iter()
        .map(|i: u32| -> Vec<f32> {
            let (x, y) = util::get_2d_coords(i, width);
            interpolate_bicubic(input, x_factor, y_factor, x, y)
        })
        .collect();

    Image::from_vec_of_vec(width, height, channels, info.alpha, data)
}

#[cfg(not(feature = "rayon"))]
fn scale_lanczos_resampling(input: &Image<f32>, output: &mut Image<f32>, x_factor: f32, y_factor: f32, size: u32) {
    for y in 0..output.info().height {
        for x in 0..output.info().width {
            let p_out = interpolate_lanczos(input, x_factor, y_factor, size, x, y);
            output.set_pixel(x, y, &p_out);
        }
    }
}

#[cfg(feature = "rayon")]
fn scale_lanczos_resampling(input: &Image<f32>, info: &ImageInfo, x_factor: f32, y_factor: f32, size: u32) -> Image<f32> {
    let img_size = info.size();
    let (width, height, channels) = info.whc();

    let data: Vec<Vec<f32>> = (0..img_size)
        .into_par_iter()
        .map(|i: u32| -> Vec<f32> {
            let (x, y) = util::get_2d_coords(i, width);
            interpolate_lanczos(input, x_factor, y_factor, size, x, y)
        })
        .collect();

    Image::from_vec_of_vec(width, height, channels, info.alpha, data)

}

fn interpolate_nearest_neighbor(input: &Image<f32>, x_factor: f32, y_factor: f32, x: u32, y: u32) -> &[f32] {
    let x_in = (((x + 1) as f32 / x_factor).ceil() - 1.0) as u32;
    let y_in = (((y + 1) as f32 / y_factor).ceil() - 1.0) as u32;

    input.get_pixel(x_in, y_in)
}

fn interpolate_bilinear(input: &Image<f32>, x_factor: f32, y_factor: f32, x: u32, y: u32) -> Vec<f32> {
    let x_in = x as f32 / x_factor;
    let y_in = y as f32 / y_factor;
    let x_1 = x_in.floor() as u32;
    let x_2 = std::cmp::min(x_in.ceil() as u32, input.info().width - 1);
    let y_1 = y_in.floor() as u32;
    let y_2 = std::cmp::min(y_in.ceil() as u32, input.info().height - 1);
    let x_weight = x_in - (x_1 as f32);
    let y_weight = y_in - (y_1 as f32);

    let p1 = input.get_pixel(x_1, y_1);
    let p2 = input.get_pixel(x_2, y_1);
    let p3 = input.get_pixel(x_1, y_2);
    let p4 = input.get_pixel(x_2, y_2);

    let mut p_out = Vec::with_capacity(input.info().channels as usize);
    for c in 0..(input.info().channels as usize) {
        p_out.push(p1[c] * x_weight * y_weight
            + p2[c] * (1.0 - x_weight) * y_weight
            + p3[c] * x_weight * (1.0 - y_weight)
            + p4[c] * (1.0 - x_weight) * (1.0 - y_weight));
    }

    p_out
}

fn interpolate_bicubic(input: &Image<f32>, x_factor: f32, y_factor: f32, x: u32, y: u32) -> Vec<f32> {
    let x_in = (x as f32) / x_factor;
    let y_in = (y as f32) / y_factor;
    let delta_x = x_in - x_in.floor();
    let delta_y = y_in - y_in.floor();

    let mut p_out = vec![0.0; input.info().channels as usize];
    for m in -1..3 {
        let x_clamp = (x_in + (m as f32)).clamp(0.0, input.info().width as f32 - 1.0) as u32;

        for n in -1..3 {
            let y_clamp = (y_in + (n as f32)).clamp(0.0, input.info().height as f32 - 1.0) as u32;
            let p_in = input.get_pixel_unchecked(x_clamp, y_clamp);
            let r = util::cubic_weighting_fn((m as f32) - delta_x)
                * util::cubic_weighting_fn(delta_y - (n as f32));

            for c in 0..(input.info().channels as usize) {
                p_out[c] += p_in[c] * r;
            }
        }
    }

    p_out
}

fn interpolate_lanczos(input: &Image<f32>, x_factor: f32, y_factor: f32, size: u32, x: u32, y: u32) -> Vec<f32> {
    let x_in = (x as f32) / x_factor;
    let y_in = (y as f32) / y_factor;
    let delta_x = x_in - x_in.floor();
    let delta_y = y_in - y_in.floor();

    let mut p_out = vec![0.0; input.info().channels as usize];
    for i in (1 - (size as i32))..(size as i32 + 1) {
        let x_clamp = (x_in + (i as f32)).clamp(0.0, input.info().width as f32 - 1.0) as u32;

        for j in (1 - (size as i32))..(size as i32 + 1) {
            let y_clamp = (y_in + (j as f32)).clamp(0.0, input.info().height as f32 - 1.0) as u32;
            let p_in = input.get_pixel_unchecked(x_clamp, y_clamp);
            let lanczos = util::lanczos_kernel(delta_x - (i as f32), size as f32)
                * util::lanczos_kernel(delta_y - (j as f32), size as f32);

            for c in 0..(input.info().channels as usize) {
                p_out[c] += p_in[c] * lanczos;
            }
        }
    }

    p_out
}