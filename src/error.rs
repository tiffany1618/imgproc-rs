use std::io;

use rulinalg;

/// Type alias for `Result<T, ImgProcError`
pub type ImgProcResult<T> = Result<T, ImgProcError>;

/// Type alias for `Result<T, ImgIoError>`
pub type ImgIoResult<T> = Result<T, ImgIoError>;

/// An enum for image processing errors
#[derive(Debug)]
pub enum ImgProcError {
    InvalidArgument(String),
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
    UnsupportedFileFormat(String),
    UnsupportedColorType(String),
    IoError(io::Error),
    Other(String),
}

impl From<io::Error> for ImgIoError {
    fn from(err: io::Error) -> Self {
        ImgIoError::IoError(err)
    }
}

impl From<String> for ImgIoError {
    fn from(err: String) -> Self {
        ImgIoError::Other(err)
    }
}
