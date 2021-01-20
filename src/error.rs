use std::io;
use std::fmt;

use png;
use jpeg_decoder;
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
    UnsupportedImageFormat(String),
    IoError(io::Error),
    PngDecodingError(png::DecodingError),
    PngEncodingError(png::EncodingError),
    JpegDecoderError(jpeg_decoder::Error),
    Other(String),
}

impl From<io::Error> for ImgIoError {
    fn from(err: io::Error) -> Self {
        ImgIoError::IoError(err)
    }
}

impl From<png::DecodingError> for ImgIoError {
    fn from(err: png::DecodingError) -> Self {
        ImgIoError::PngDecodingError(err)
    }
}

impl From<png::EncodingError> for ImgIoError {
    fn from(err: png::EncodingError) -> Self {
        ImgIoError::PngEncodingError(err)
    }
}

impl From<jpeg_decoder::Error> for ImgIoError {
    fn from(err: jpeg_decoder::Error) -> Self {
        ImgIoError::JpegDecoderError(err)
    }
}

impl From<String> for ImgIoError {
    fn from(err: String) -> Self {
        ImgIoError::Other(err)
    }
}
