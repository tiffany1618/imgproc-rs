use crate::image::{Number, Image, ImageInfo, BaseImage};
use crate::error::{ImgProcResult, ImgProcError};

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

    for i in 0..height {
        for j in 0..width {
            output.set_pixel(i, j, input.get_pixel(i + x, j + y));
        }
    }

    Ok(output)
}

/// Aligns the top left corner of `front` onto the location `(x, y)` on `back` and superimposes
/// the two images with weight `alpha` for pixel values of `back` and weight 1 - `alpha` for
/// pixel values of `front`
pub fn superimpose(back: &Image<f64>, front: &Image<f64>, x: u32, y: u32, alpha: f64) -> ImgProcResult<Image<f64>> {
    if back.info().channels != front.info().channels {
        return Err(ImgProcError::InvalidArgError("input images do not have the same number of channels".to_string()));
    } else if alpha < 0.0 || alpha > 1.0 {
        return Err(ImgProcError::InvalidArgError("alpha is not in range [0.0, 1.0]".to_string()));
    }

    let mut output = back.clone();
    let width = std::cmp::min(x + front.info().width, back.info().width);
    let height = std::cmp::min(y + front.info().height, back.info().height);

    for i in x..width {
        for j in y..height {
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
    if back.info().channels != front.info().channels {
        return Err(ImgProcError::InvalidArgError("input images do not have the same number of channels".to_string()));
    }

    let mut output = back.clone();
    let width = std::cmp::min(x + front.info().width, back.info().width);
    let height = std::cmp::min(y + front.info().height, back.info().height);

    for i in x..width {
        for j in y..height {
            output.set_pixel(i, j, front.get_pixel(i - x, j - y));
        }
    }

    Ok(output)
}

/////////////
// Scaling
/////////////

/// Scales an image by a factor of `factor` using nearest neighbor interpolation
pub fn scale_nearest_neighbor<T: Number>(input: &Image<T>, factor: f64) -> ImgProcResult<Image<T>> {
    if factor <= 0.0 {
        return Err(ImgProcError::InvalidArgError("factor must be positive".to_string()));
    }

    let width = (input.info().width as f64 * factor).round() as u32;
    let height = (input.info().height as f64 * factor).round() as u32;
    let mut output = Image::blank(ImageInfo::new(width, height,
                                                 input.info().channels, input.info().alpha));

    for x in 0..width {
        for y in 0..height {
            let index_x = (((x + 1) as f64 / factor).ceil() - 1.0) as u32;
            let index_y = (((y + 1) as f64 / factor).ceil() - 1.0) as u32;
            output.set_pixel(x, y, input.get_pixel(index_x, index_y));
        }
    }

    Ok(output)
}

/// Scales an image by a factor of `factor` using bilinear interpolation
pub fn scale_bilinear<T: Number>(input: &Image<T>, factor: f64) -> ImgProcResult<Image<T>> {
    if factor <= 0.0 {
        return Err(ImgProcError::InvalidArgError("factor must be positive".to_string()));
    }

    let width = (input.info().width as f64 * factor).round() as u32;
    let height = (input.info().height as f64 * factor).round() as u32;
    let mut output = Image::blank(ImageInfo::new(width, height,
                                                 input.info().channels, input.info().alpha));

    for x in 0..width {
        for y in 0..height {
            // TODO
        }
    }

    Ok(output)
}
/// Scales an image by a factor of `factor` using bicubic interpolation
pub fn scale_bicubic<T: Number>(input: &Image<T>, factor: f64) -> ImgProcResult<Image<T>> {
    if factor <= 0.0 {
        return Err(ImgProcError::InvalidArgError("factor must be positive".to_string()));
    }

    let width = (input.info().width as f64 * factor).round() as u32;
    let height = (input.info().height as f64 * factor).round() as u32;
    let mut output = Image::blank(ImageInfo::new(width, height,
                                                 input.info().channels, input.info().alpha));

    for x in 0..width {
        for y in 0..height {
            // TODO
        }
    }

    Ok(output)
}
/// Scales an image by a factor of `factor` using Sinc resampling
pub fn scale_sinc<T: Number>(input: &Image<T>, factor: f64) -> ImgProcResult<Image<T>> {
    if factor <= 0.0 {
        return Err(ImgProcError::InvalidArgError("factor must be positive".to_string()));
    }

    let width = (input.info().width as f64 * factor).round() as u32;
    let height = (input.info().height as f64 * factor).round() as u32;
    let mut output = Image::blank(ImageInfo::new(width, height,
                                                 input.info().channels, input.info().alpha));

    for x in 0..width {
        for y in 0..height {
            // TODO
        }
    }

    Ok(output)
}
