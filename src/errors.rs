use std::io;

use png;
use jpeg_decoder;

/// An enum for image errors
pub enum ImageError {
    IoError(io::Error),
    PngDecodingError(png::DecodingError),
    PngEncodingError(png::EncodingError),
    JpegDecoderError(jpeg_decoder::Error),
    Other(String),
}

impl From<io::Error> for ImageError {
    fn from(err: io::Error) -> Self {
        ImageError::IoError(err)
    }
}

impl From<png::DecodingError> for ImageError {
    fn from(err: png::DecodingError) -> Self {
        ImageError::PngDecodingError(err)
    }
}

impl From<png::EncodingError> for ImageError {
    fn from(err: png::EncodingError) -> Self {
        ImageError::PngEncodingError(err)
    }
}

impl From<jpeg_decoder::Error> for ImageError {
    fn from(err: jpeg_decoder::Error) -> Self {
        ImageError::JpegDecoderError(err)
    }
}

impl From<String> for ImageError {
    fn from(err: String) -> Self {
        ImageError::Other(err)
    }
}
