use crate::error::{ImgProcResult, ImgProcError};
use crate::image::Number;

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

pub(crate) fn check_grayscale(channels: u8, alpha: bool) -> ImgProcResult<()> {
    if (alpha && channels != 2) || (!alpha && channels != 1) {
        return Err(ImgProcError::InvalidArgError("input is not a grayscale image".to_string()));
    }

    Ok(())
}