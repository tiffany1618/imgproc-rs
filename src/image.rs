use crate::util::Number;

#[derive(Debug, Clone, PartialEq)]
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

    // Return all channels as slice
    pub fn channels(&self) -> &[T] {
        &self.channels
    }

    // Return all channels as mutable slice
    pub fn channels_mut(&mut self) -> &mut [T] {
        &mut self.channels
    }

    // Return all channels except last channel as slice
    pub fn channels_no_alpha(&self) -> &[T] {
        &self.channels[..(self.num_channels as usize)]
    }

    // Return last channel if it exists
    pub fn alpha(&self) -> T {
        self.channels[(self.num_channels - 1) as usize]
    }

    // Apply function f to all channels
    pub fn map<S: Number, F>(&self, f: F) -> Pixel<S>
        where F: Fn(T) -> S {
        let mut channels_out = Vec::new();

        for channel in self.channels.iter() {
            channels_out.push(f(*channel));
        }

        Pixel {
            num_channels: self.num_channels,
            channels: channels_out,
        }
    }

    // Apply function f to all channels except alpha channel
    // Apply function g to alpha channel
    pub fn map_alpha<S: Number, F, G>(&self, f: F, g: G) -> Pixel<S>
        where F: Fn(T) -> S,
              G: Fn(T) -> S {
        let mut channels_out = Vec::new();

        for p in self.channels_no_alpha().iter() {
            channels_out.push(f(*p));
        }

        channels_out.push(g(self.alpha()));

        Pixel {
            num_channels: self.num_channels,
            channels: channels_out,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image<T: Number> {
    width: u32,
    height: u32,
    channels: u8,
    alpha: bool,
    pixels: Vec<Pixel<T>>,
}

impl<T: Number> Image<T> {
    pub fn new(width: u32, height: u32, channels: u8, alpha: bool, data: &[T]) -> Self {
        let mut pixels = Vec::new();
        let size = (width * height * channels as u32) as usize;
        for i in (0..size).step_by(channels as usize) {
            let pixel = Pixel::new(&data[i..(i + channels as usize)]);
            pixels.push(pixel);
        }

        Image { width, height, channels, alpha, pixels }
    }

    pub fn blank(width: u32, height: u32, channels: u8, alpha: bool) -> Self {
        Image {
            width,
            height,
            channels,
            alpha,
            pixels: vec![Pixel::blank(channels); (width * height) as usize],
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn dimensions_with_channels(&self) -> (u32, u32, u8) {
        (self.width, self.height, self.channels)
    }

    pub fn has_alpha(&self) -> bool {
        self.alpha
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

    // Return pixel as mutable slice
    pub fn pixel_mut(&mut self, x: u32, y: u32) -> &mut [T] {
        self.pixels[(y * self.width + x) as usize].channels_mut()
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &Pixel<T> {
        &self.pixels[(y * self.width + x) as usize]
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, p: Pixel<T>) {
        self.pixels[(y * self.width + x) as usize] = p;
    }

    // Apply function f to all pixels
    pub fn map_pixels<S: Number, F>(&self, f: F) -> Image<S>
        where F: Fn(&[T]) -> Vec<S> {
        let (width, height) = self.dimensions();
        let mut pixels = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let p = Pixel::new(&f(&self.get_pixel(x, y).channels()));
                pixels.push(p);
            }
        }

        Image {
            width,
            height,
            channels: pixels[0].num_channels(),
            alpha: self.alpha,
            pixels,
        }
    }

    // If image has alpha channel, apply function f to non-alpha portion of all pixels, and
    // function g to alpha channel of all pixels;
    // if image has no alpha channel, apply function f to all pixels
    pub fn map_pixels_if_alpha<S: Number, F, G>(&self, f: F, g: G) -> Image<S>
        where F: Fn(&[T]) -> Vec<S>,
              G: Fn(T) -> S {
        let (width, height) = self.dimensions();
        let mut pixels = Vec::new();

        if !self.alpha {
            return self.map_pixels(f);
        }

        for y in 0..height {
            for x in 0..width {
                let mut v = f(&self.get_pixel(x, y).channels_no_alpha());
                v.push(g(self.get_pixel(x, y).alpha()));
                pixels.push(Pixel::new(&v));
            }
        }

        Image {
            width,
            height,
            channels: pixels[0].num_channels(),
            alpha: self.alpha,
            pixels,
        }
    }

    // Apply function f to all channels of all pixels
    pub fn map_channels<S: Number, F>(&self, f: F) -> Image<S>
        where F: Fn(T) -> S {
        let (width, height, channels) = self.dimensions_with_channels();
        let mut pixels = Vec::new();

        for y in 0..height {
            for x in 0..width {
                pixels.push(self.get_pixel(x, y).map(&f));
            }
        }

        Image {
            width,
            height,
            channels,
            alpha: self.alpha,
            pixels,
        }
    }

    // If image has alpha channel, apply function f to all non-alpha channels of all pixels, and
    // function g to alpha channel;
    // If image has no alpha channel, apply function f to all channels of all pixels
    pub fn map_channels_if_alpha<S: Number, F, G>(&self, f: F, g: G) -> Image<S>
        where F: Fn(T) -> S,
              G: Fn(T) -> S {
        let (width, height, channels) = self.dimensions_with_channels();
        let mut pixels = Vec::new();

        if !self.alpha {
            return self.map_channels(f);
        }

        for y in 0..height {
            for x in 0..width {
                pixels.push(self.get_pixel(x, y).map_alpha(&f, &g))
            }
        }

        Image {
            width,
            height,
            channels,
            alpha: self.alpha,
            pixels,
        }
    }

    // Apply function f to each channel of index channel_index of each pixel
    pub fn edit_channel<F>(&mut self, f: F, channel_index: usize)
        where F: Fn(T) -> T {
        let (width, height) = self.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel = self.pixel_mut(x, y);
                pixel[channel_index] = f(pixel[channel_index]);
            }
        }
    }
}