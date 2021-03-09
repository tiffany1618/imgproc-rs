use crate::error::{ImgProcResult, ImgProcError};
use crate::image::{Number, Image, BaseImage};

pub(crate) fn check_channels(channels: u8, len: usize) {
    if channels != len as u8 {
        panic!("invalid pixel length: the number of channels is {}, \
                but the pixel length is {}", channels, len);
    }
}

pub(crate) fn check_xy(x: u32, y: u32, width: u32, height: u32) {
    if x >= width {
        panic!("index out of bounds: the width is {}, but the x index is {}", width, x)
    }
    if y >= height {
        panic!("index out of bounds: the height is {}, but the y index is {}", height, y)
    }
}

pub(crate) fn check_odd<T: Number>(val: T, name: &str) -> ImgProcResult<()> {
    if val % 2.into() == 0.into() {
        return Err(ImgProcError::InvalidArgError(format!("{} must be odd", name)));
    }

    Ok(())
}

pub(crate) fn check_even<T: Number>(val: T, name: &str) -> ImgProcResult<()> {
    if val % 2.into() != 0.into() {
        return Err(ImgProcError::InvalidArgError(format!("{} must be even", name)));
    }

    Ok(())
}

pub(crate) fn check_non_neg<T: Number>(val: T, name: &str) -> ImgProcResult<()> {
    if val < 0.into() {
        return Err(ImgProcError::InvalidArgError(format!("{} must be non-negative", name)));
    }

    Ok(())
}

pub(crate) fn check_equal<T: std::cmp::PartialEq>(val_1: T, val_2: T, name: &str) -> ImgProcResult<()> {
    if val_1 != val_2 {
        return Err(ImgProcError::InvalidArgError(format!("{} must be equal", name)));
    }

    Ok(())
}

pub(crate) fn check_square(val: f64, name: &str) -> ImgProcResult<()> {
    if val.sqrt() % 1.0 != 0.0 {
        return Err(ImgProcError::InvalidArgError(format!("{} must be square", name)));
    }

    Ok(())
}

pub(crate) fn check_in_range<T: Number>(val: T, min: T, max: T, name: &str) -> ImgProcResult<()> {
    if val < min || val > max {
        return Err(ImgProcError::InvalidArgError(format!("{} must be between {} and {} (inclusive)", name, min, max)));
    }

    Ok(())
}

pub(crate) fn check_grayscale<T: Number>(input: &Image<T>) -> ImgProcResult<()> {
    if (input.info().alpha && input.info().channels != 2) || (!input.info().alpha && input.info().channels != 1) {
        return Err(ImgProcError::InvalidArgError("input is not a grayscale image".to_string()));
    }

    Ok(())
}