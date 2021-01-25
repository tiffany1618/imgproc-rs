use crate::image::ImageInfo;

/// A trait for valid image channel types
pub trait Number:
    std::marker::Copy
    + std::fmt::Display
    + std::ops::Add<Output=Self>
    + std::ops::Sub<Output=Self>
    + std::ops::Mul<Output=Self>
    + std::ops::Div<Output=Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + From<u8>
    where Self: std::marker::Sized {}

impl<T> Number for T
    where T:
    std::marker::Copy
    + std::fmt::Display
    + std::ops::Add<Output=T>
    + std::ops::Sub<Output=T>
    + std::ops::Mul<Output=T>
    + std::ops::Div<Output=T>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + From<u8> {}

/// A trait for a base image
pub trait BaseImage<T: Number> {
    /// Returns the image information
    fn info(&self) -> ImageInfo;

    /// Returns a PixelSlice representing the pixel located at `(x, y)`
    fn get_pixel(&self, x: u32, y: u32) -> &[T];
}