use std::io;
use std::fmt;

use png;

#[derive(Debug, Clone)]
pub struct FileFormatError;

impl fmt::Display for FileFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unsupported file format")
    }
}

pub enum ImageError {
    IoError(io::Error),
    PngDecodingError(png::DecodingError),
    PngEncodingError(png::EncodingError),
    FormatError(FileFormatError),
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

impl From<FileFormatError> for ImageError {
    fn from(err: FileFormatError) -> Self {
        ImageError::FormatError(err)
    }
}

impl From<String> for ImageError {
    fn from(err: String) -> Self {
        ImageError::Other(err)
    }
}
