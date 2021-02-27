use crate::image::{Number, ImageInfo, BaseImage};
use crate::error::check_xy;

/// A struct representing a part of an image
#[derive(Debug, Clone)]
pub struct SubImage<'a, T: Number> {
    info: ImageInfo,
    data: Vec<&'a [T]>,
}

impl<'a, T: Number> SubImage<'a, T> {
    /// Creates a new `SubImage<T>`
    pub fn new(width: u32, height: u32, channels: u8, alpha: bool, data: Vec<&'a [T]>) -> Self {
        SubImage {
            info: ImageInfo { width, height, channels, alpha },
            data,
        }
    }

    /// Returns all data as a slice of slices
    pub fn data(&self) -> &[&[T]] {
        &self.data[..]
    }

    /// Converts all data to a vector
    pub fn to_vec(&self) -> Vec<T> {
        let mut data = Vec::new();

        for i in 0..(self.info.size() as usize) {
            data.extend_from_slice(&self[i]);
        }

        data
    }
}

impl<T: Number> BaseImage<T> for SubImage<'_, T> {
    fn info(&self) -> ImageInfo {
        self.info
    }

    fn get_pixel(&self, x: u32, y: u32) -> &[T] {
        check_xy(x, y, self.info.width, self.info.height);

        &self[(y * self.info.width + x) as usize]
    }
}

impl<T: Number> std::ops::Index<usize> for SubImage<'_, T> {
    type Output = [T];

    fn index(&self, i: usize) -> &Self::Output {
        self.data[i]
    }
}