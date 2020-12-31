use crate::util::Primitive;

pub struct Pixel<T: Primitive> {
    num_channels: usize,
    channels: Vec<T>,
}

impl<T: Primitive> Pixel<T> {
    pub fn new(channels: &[T]) -> Self {
        Pixel {
            num_channels: channels.len(),
            channels: channels.to_vec(),
        }
    }

    pub fn blank(num_channels: usize) -> Self {
        Pixel {
            num_channels,
            channels: vec![0 as T; num_channels],
        }
    }

    // Return all channels as vector
    pub fn channels(&self) -> Vec<T> {
        self.channels
    }

    // Return alpha channel
    pub fn alpha(&self) -> T {
        self.channels[self.num_channels - 1]
    }

    // Return all channels except alpha channel as slice
    pub fn channels_no_alpha(&self) -> Vec<T> {
        self.channels[0..self.num_channels].to_vec()
    }

    // Apply function f to all channels
    pub fn map<S: Primitive>(&self, f: fn(T) -> S) -> Pixel<S> {
        let mut channels = Vec::new();

        for i in 0..self.num_channels {
            channels.push(f(self.channels[0]));
        }

        Pixel {
            num_channels: self.num_channels,
            channels,
        }
    }

    // Apply function f to all channels except alpha channel
    pub fn map_no_alpha<S: Primitive>(&self, f: fn(T) -> S) -> Pixel<S> {
        let mut channels = Vec::new();

        for p in self.channels_no_alpha().iter() {
            channels.push(f(*p));
        }

        channels.push(self.alpha() as S);

        Pixel {
            num_channels: self.num_channels,
            channels,
        }
    }
}

pub struct Image<T: Primitive> {
    width: u32,
    height: u32,
    pixels: Vec<Pixel<T>>,
}

impl<T: Primitive> Image<T> {
    pub fn new(width: u32, height: u32, channels: u32, data: &[T]) -> Self {
        let pixels = Vec::new();
        let size = width * height * channels;
        for i in (0..size).step_by(channels as usize) {
            let pixel = Pixel::new(data[i..(i + channels)]);
            pixels.push(pixel);
        }

        Image {width, height, pixels}
    }

    pub fn blank(width: u32, height: u32, channels: usize) -> Self {
        Image {
            width,
            height,
            pixels: vec![Pixel::blank(channels); (width * height) as usize],
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn pixels(&self) -> Vec<Pixel<T>> {
        self.pixels
    }

    pub fn pixels_as_vector(&self) -> Vec<T> {
        let pixels_vec = Vec::new();

        for i in self.pixels.iter() {
            for j in i.channels().iter() {
                pixels_vec.push(*j);
            }
        }

        pixels_vec
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel<T>{
        self.pixels[(y * self.width + x) as usize]
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, p: Pixel<T>) {
        self.pixels[(y * self.width + x) as usize] = p;
    }

    // Apply function f to all pixels
    // alpha = true to include alpha channel, false otherwise
    pub fn map_pixels<S: Primitive>(&self, f: fn(&[T]) -> Vec<S>, alpha: bool) -> Image<S> {
        let (width, height) = self.dimensions();
        let pixels = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if alpha {
                    let p = Pixel::new(&f(&self.get_pixel(x, y).channels()));
                    pixels.push(p);
                } else {
                    let p = Pixel::new(&f(&self.get_pixel(x, y).channels_no_alpha()));
                    pixels.push(p)
                }
            }
        }

        Image {width, height, pixels}
    }

    // Apply function f to all channels of all pixels
    // alpha = true to apply to alpha channel, false otherwise
    pub fn map_channels<S: Primitive>(&self, f: fn(T) -> S, alpha: bool) -> Image<S> {
        let (width, height) = self.dimensions();
        let pixels = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if alpha {
                    pixels.push(self.get_pixel(x, y).map(f));
                } else {
                    pixels.push(self.get_pixel(x, y).map_no_alpha(f));
                }
            }
        }

        Image {width, height, pixels}
    }
}