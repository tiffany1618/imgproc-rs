//! A module for image transformation operations

use crate::{math, error};
use crate::image::{Number, Image, ImageInfo, BaseImage};
use crate::error::{ImgProcResult, ImgProcError};
use crate::enums::{Scale, Refl};

/// Crops an image to a rectangle with upper left corner located at `(x, y)` with width `width`
/// and height `height`
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

/// Aligns the top left corner of `front` onto the location `(x, y)` on `back` and superimposes
/// the two images with weight `alpha` for pixel values of `back` and weight 1 - `alpha` for
/// pixel values of `front`
pub fn superimpose(back: &Image<f64>, front: &Image<f64>, x: u32, y: u32, alpha: f64) -> ImgProcResult<Image<f64>> {
    error::check_equal(back.info().channels, front.info().channels, "image channels")?;
    if alpha < 0.0 || alpha > 1.0 {
        return Err(ImgProcError::InvalidArgError("alpha is not in range [0.0, 1.0]".to_string()));
    }

    let mut output = back.clone();
    let width = std::cmp::min(x + front.info().width, back.info().width);
    let height = std::cmp::min(y + front.info().height, back.info().height);

    for j in y..height {
        for i in x..width {
            let mut pixel_new = Vec::new();
            let pixel_back = back.get_pixel(i, j);
            let pixel_front = front.get_pixel(i - x, j - y);
            for k in 0..(output.info().channels as usize) {
                pixel_new.push(alpha * pixel_back[k] + (1.0 - alpha) * pixel_front[k]);
            }

            output.set_pixel(i, j, &pixel_new);
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
pub fn scale(input: &Image<f64>, x_factor: f64, y_factor: f64, method: Scale) -> ImgProcResult<Image<f64>> {
    error::check_non_neg(x_factor, "x_factor")?;
    error::check_non_neg(y_factor, "y_factor")?;


    let width = (input.info().width as f64 * x_factor).round() as u32;
    let height = (input.info().height as f64 * y_factor).round() as u32;
    let mut output = Image::blank(ImageInfo::new(width, height,
                                                 input.info().channels, input.info().alpha));

    match method {
        Scale::NearestNeighbor => {
            for y in 0..height {
                for x in 0..width {
                    let index_x = (((x + 1) as f64 / x_factor).ceil() - 1.0) as u32;
                    let index_y = (((y + 1) as f64 / y_factor).ceil() - 1.0) as u32;
                    output.set_pixel(x, y, input.get_pixel(index_x, index_y));
                }
            }
        },
        Scale::Bilinear => {
            for y in 0..height {
                for x in 0..width {
                    let x_f = x as f64 / x_factor;
                    let y_f = y as f64 / y_factor;
                    let x_1 = x_f.floor() as u32;
                    let x_2 = std::cmp::min(x_f.ceil() as u32, input.info().width - 1);
                    let y_1 = y_f.floor() as u32;
                    let y_2 = std::cmp::min(y_f.ceil() as u32, input.info().height - 1);
                    let x_weight = x_f - (x_1 as f64);
                    let y_weight = y_f - (y_1 as f64);

                    let p1 = input.get_pixel(x_1, y_1);
                    let p2 = input.get_pixel(x_2, y_1);
                    let p3 = input.get_pixel(x_1, y_2);
                    let p4 = input.get_pixel(x_2, y_2);

                    let mut pixel = Vec::new();
                    for c in 0..(output.info().channels as usize) {
                        pixel.push(p1[c] * x_weight * y_weight
                            + p2[c] * (1.0 - x_weight) * y_weight
                            + p3[c] * x_weight * (1.0 - y_weight)
                            + p4[c] * (1.0 - x_weight) * (1.0 - y_weight));
                    }

                    output.set_pixel(x, y, &pixel);
                }
            }
        },
    }

    Ok(output)
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
pub fn rotate(input: &Image<f64>, degrees: f64) -> ImgProcResult<Image<f64>> {
    let (w_in, h_in) = input.info().wh();
    let (sin, cos) = degrees.to_radians().sin_cos();

    // Center coordinates
    let x = w_in / 2;
    let y = h_in / 2;

    let mat = [cos, -sin, sin, cos];

    // Compute dimensions of output image
    let coords1 = math::vector_mul(&mat, &[-(x as f64), y as f64])?;
    let coords2 = math::vector_mul(&mat, &[(w_in - x) as f64, y as f64])?;
    let coords3 = math::vector_mul(&mat, &[-(x as f64), (y as f64) - (h_in as f64)])?;
    let coords4 = math::vector_mul(&mat, &[(w_in - x) as f64, (y as f64) - (h_in as f64)])?;

    let x_max = math::max_4(coords1[0], coords2[0], coords3[0], coords4[0]);
    let x_min = math::min_4(coords1[0], coords2[0], coords3[0], coords4[0]);
    let y_max = math::max_4(coords1[1], coords2[1], coords3[1], coords4[1]);
    let y_min = math::min_4(coords1[1], coords2[1], coords3[1], coords4[1]);

    let w_out = (x_max - x_min) as u32;
    let h_out = (y_max - y_min) as u32;

    let mut output = Image::blank(ImageInfo::new(w_out, h_out,
                                                 input.info().channels, input.info().alpha));

    for j in 0..h_in {
        for i in 0..w_in {
            let x1 = (i as f64) - (x as f64);
            let y1 = (y as f64) - (j as f64);

            let mut coords = math::vector_mul(&mat, &[x1, y1])?;

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
pub fn shear(input: &Image<f64>, shear_x: f64, shear_y: f64) -> ImgProcResult<Image<f64>> {
    let (w_in, h_in) = input.info().wh();
    let offset_x = (h_in as f64 * shear_x).abs();
    let offset_y = (w_in as f64 * shear_y).abs();
    let w_out = w_in + offset_x as u32;
    let h_out = offset_y as u32 + h_in;
    let mut output = Image::blank(ImageInfo::new(w_out, h_out,
                                                 input.info().channels, input.info().alpha));

    // Negative sign to give the conventional orientation for a positive shear, since the image
    // coordinates are flipped from conventional coordinates (i.e. (0,0) is in the top left corner
    // instead of the bottom left corner)
    let mat = [1.0, -shear_x, -shear_y, 1.0];

    for y in 0..h_in {
        for x in 0..w_in {
            let mut coords = math::vector_mul(&mat, &[x as f64, y as f64])?;

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
