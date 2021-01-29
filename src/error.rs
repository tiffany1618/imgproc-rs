use std::io;

use rulinalg;

/// Type alias for `Result<T, ImgProcError`
pub type ImgProcResult<T> = Result<T, ImgProcError>;

/// Type alias for `Result<T, ImgIoError>`
pub type ImgIoResult<T> = Result<T, ImgIoError>;

/// An enum for image processing errors
#[derive(Debug)]
pub enum ImgProcError {
    InvalidArgError(String),
    RulinalgError(rulinalg::error::Error),
}

impl From<rulinalg::error::Error> for ImgProcError {
    fn from(err: rulinalg::error::Error) -> Self {
        ImgProcError::RulinalgError(err)
    }
}

/// An enum for image i/o errors
#[derive(Debug)]
pub enum ImgIoError {
    UnsupportedFileFormatError(String),
    UnsupportedColorTypeError(String),
    IoError(io::Error),
    ImageReaderError(image::error::ImageError),
    ImageWriteError(String),
    OtherError(String),
}

impl From<io::Error> for ImgIoError {
    fn from(err: io::Error) -> Self {
        ImgIoError::IoError(err)
    }
}

impl From<image::error::ImageError> for ImgIoError {
    fn from(err: image::error::ImageError) -> Self {
        ImgIoError::ImageReaderError(err)
    }
}

impl From<String> for ImgIoError {
    fn from(err: String) -> Self {
        ImgIoError::OtherError(err)
    }
}
