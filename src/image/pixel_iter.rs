use crate::image::{Image, BaseImage, Number};

/// A struct representing a pixel iterator for an image
pub struct PixelIter<'a, T: Number> {
    image: &'a Image<T>,
    index: usize,
}

pub struct PixelIter2d<'a, T: Number> {
    image: &'a Image<T>,
    pub x: u32,
    pub y: u32,
}

pub struct Iter2d {
    x: u32,
    y: u32,
    x_max: u32,
    y_max: u32,
}

impl Iter2d {
    pub fn new(x_max: u32, y_max: u32) -> Self {
        Iter2d {
            x: 0,
            y: 0,
            x_max,
            y_max,
        }
    }
}

impl Iterator for Iter2d {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;

        if self.x >= self.x_max {
            if self.y == self.y_max - 1 {
                return None;
            } else {
                self.x = 0;
                self.y += 1;
            }
        }

        Some((self.x - 1, self.y))
    }
}

// impl IndexedParallelIterator for Iter2d {
//     fn len(&self) -> usize {
//         unimplemented!()
//     }
//
//     fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> <C as Consumer<Self::Item>>::Result {
//         unimplemented!()
//     }
//
//     fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> <CB as ProducerCallback<Self::Item>>::Output {
//         unimplemented!()
//     }
// }
//
// impl ParallelIterator for Iter2d {
//     type Item = (u32, u32);
//
//     fn drive_unindexed<C>(self, consumer: C) -> <C as Consumer<Self::Item>>::Result where
//         C: UnindexedConsumer<Self::Item> {
//
//     }
// }

impl<T: Number> IntoIterator for &Image<T> {
    type Item = (u32, u32);
    type IntoIter = Iter2d;

    fn into_iter(self) -> Self::IntoIter {
        Iter2d::new(self.info.width, self.info.height)
    }
}

impl<'a, T: Number> PixelIter<'a, T> {
    pub fn new(image: &'a Image<T>) -> Self {
        PixelIter {
            image,
            index: 0,
        }
    }
}

impl<'a, T: Number> Iterator for PixelIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        if self.index >= self.image.info().size() as usize {
            None
        } else {
            Some(&self.image[self.index-1])
        }
    }
}

impl<'a, T: Number> PixelIter2d<'a, T> {
    pub fn new(image: &'a Image<T>) -> Self {
        PixelIter2d {
            image,
            x: 0,
            y: 0,
        }
    }
}

impl<'a, T: Number> Iterator for PixelIter2d<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;

        if self.x >= self.image.info().width {
            if self.y == self.image.info().height - 1 {
                return None;
            } else {
                self.x = 0;
                self.y += 1;
            }
        }

        Some(self.image.get_pixel(self.x - 1, self.y))
    }
}
