use crate::util::Number;

#[derive(Debug, Clone)]
pub struct Pixel<T: Number> {
    num_channels: u8,
    channels: Vec<T>,
}

impl<T: Number> Pixel<T> {
    pub fn new(channels: &[T]) -> Self {
        Pixel {
            num_channels: channels.len() as u8,
            channels: channels.to_vec(),
        }
    }

    pub fn blank(num_channels: u8) -> Self {
        Pixel {
            num_channels,
            channels: vec![0.into(); num_channels as usize],
        }
    }

    pub fn num_channels(&self) -> u8 {
        self.num_channels
    }

    pub fn channels(&self) -> &[T] {
        &self.channels
    }

    // Return alpha channel
    pub fn alpha(&self) -> T {
        self.channels[(self.num_channels - 1) as usize]
    }

    // Return all channels except alpha channel
    pub fn channels_no_alpha(&self) -> &[T] {
        &self.channels[0..(self.num_channels as usize)]
    }

    // Apply function f to all channels
    pub fn map<S: Number>(&self, f: fn(T) -> S) -> Pixel<S> {
        let mut channels = Vec::new();

        for i in 0..self.num_channels {
            channels.push(f(self.channels[i as usize]));
        }

        Pixel {
            num_channels: self.num_channels,
            channels,
        }
    }

    // Apply function f to all channels except alpha channel
    pub fn map_no_alpha<S: Number + From<T>>(&self, f: fn(T) -> S) -> Pixel<S> {
        let mut channels = Vec::new();

        for p in self.channels_no_alpha().iter() {
            channels.push(f(*p));
        }

        channels.push(self.alpha().into());

        Pixel {
            num_channels: self.num_channels,
            channels,
        }
    }
}

#[derive(Debug)]
pub struct Image<T: Number> {
    width: u32,
    height: u32,
    channels: u8,
    pixels: Vec<Pixel<T>>,
}

impl<T: Number> Image<T> {
    pub fn new(width: u32, height: u32, channels: u8, data: &[T]) -> Self {
        let mut pixels = Vec::new();
        let size = (width * height * channels as u32) as usize;
        for i in (0..size).step_by(channels as usize) {
            let pixel = Pixel::new(&data[i..(i + channels as usize)]);
            pixels.push(pixel);
        }

        Image {width, height, channels, pixels}
    }

    pub fn blank(width: u32, height: u32, channels: u8) -> Self {
        Image {
            width,
            height,
            channels,
            pixels: vec![Pixel::blank(channels); (width * height) as usize],
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn dimensions_with_channels(&self) -> (u32, u32, u8) {
        (self.width, self.height, self.channels)
    }

    pub fn pixels(&self) -> &[Pixel<T>] {
        &self.pixels
    }

    pub fn pixels_as_vector(&self) -> Vec<T> {
        let mut pixels_vec = Vec::new();

        for i in self.pixels.iter() {
            for j in i.channels().iter() {
                pixels_vec.push(*j);
            }
        }

        pixels_vec
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &Pixel<T>{
        &self.pixels[(y * self.width + x) as usize]
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, p: Pixel<T>) {
        self.pixels[(y * self.width + x) as usize] = p;
    }

    // Apply function f to all pixels
    // alpha = true to include alpha channel, false otherwise
    pub fn map_pixels<S: Number + From<T>>(&self, f: fn(&[T]) -> Vec<S>, alpha: bool) -> Image<S> {
        let (width, height) = self.dimensions();
        let mut pixels = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if alpha {
                    let p = Pixel::new(&f(&self.get_pixel(x, y).channels()));
                    pixels.push(p);
                } else {
                    let mut v = f(&self.get_pixel(x, y).channels_no_alpha());
                    v.push(self.get_pixel(x, y).alpha().into());
                    pixels.push(Pixel::new(&v));
                }
            }
        }

        Image {
            width,
            height,
            channels: pixels[0].channels().len() as u8,
            pixels,
        }
    }

    // Apply function f to all channels of all pixels
    // alpha = true to apply to alpha channel, false otherwise
    pub fn map_channels<S: Number + From<T>>(&self, f: fn(T) -> S, alpha: bool) -> Image<S> {
        let (width, height) = self.dimensions();
        let mut pixels = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if alpha {
                    pixels.push(self.get_pixel(x, y).map(f));
                } else {
                    pixels.push(self.get_pixel(x, y).map_no_alpha(f));
                }
            }
        }

        Image {
            width,
            height,
            channels: pixels[0].channels().len() as u8,
            pixels,
        }
    }
}