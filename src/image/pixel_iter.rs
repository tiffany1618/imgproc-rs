use crate::image::{Image, BaseImage, Number};

/// A struct representing a pixel iterator for an image. `next()` returns a tuple containing the
/// x-coordinate, y-coordinate, and a slice representing the pixel at that coordinate, in that
/// order.
///
/// # Examples
/// ```rust
/// # fn main() {
/// use imgproc_rs::image::{Image, BaseImage};
///
/// // Create an image
/// let img = Image::from_vec(2, 2, 3, false, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
///
/// // Print pixels with corresponding coordinates using the pixel iterator
/// for vals in img.into_iter() {
///     print!("(x: {}, y: {}), pixel: (", vals.0, vals.1);
///
///     for i in 0..(img.info().channels as usize) {
///         print!("{}, ", vals.2[i]);
///     }
///
///     print!(")");
///     println!();
/// }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PixelIter<'a, T: Number> {
    image: &'a Image<T>,
    x: u32,
    y: u32,
    x_max: u32,
    y_max: u32,
}

impl<'a, T: Number> PixelIter<'a, T> {
    pub fn new(image: &'a Image<T>) -> Self {
        PixelIter {
            image,
            x: 0,
            y: 0,
            x_max: image.info().width - 1,
            y_max: image.info().height - 1,
        }
    }
}

impl<'a, T: Number> Iterator for PixelIter<'a, T> {
    type Item = (u32, u32, &'a [T]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.x_max {
            if self.y >= self.y_max {
                return None;
            } else {
                self.x = 0;
                self.y += 1;
            }
        }

        let temp_x = self.x;
        let temp_y = self.y;
        self.x += 1;

        Some((temp_x, temp_y, self.image.get_pixel(temp_x, temp_y)))
    }
}

impl<'a, T: Number> IntoIterator for &'a Image<T> {
    type Item = (u32, u32, &'a [T]);
    type IntoIter = PixelIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        PixelIter::new(&self)
    }
}