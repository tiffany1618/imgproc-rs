use crate::util::Number;

/// A Pixel representation
#[derive(Debug, Clone, PartialEq)]
pub struct Pixel<T: Number> {
    num_channels: u8,
    channels: Vec<T>,
}

impl<T: Number> Pixel<T> {
    /// Creates a new `Pixel` from a slice
    pub fn new(channels: &[T]) -> Self {
        Pixel {
            num_channels: channels.len() as u8,
            channels: channels.to_vec(),
        }
    }

    /// Creates a `Pixel` populated with zeroes
    pub fn blank(num_channels: u8) -> Self {
        Pixel {
            num_channels,
            channels: vec![0.into(); num_channels as usize],
        }
    }

    /// Returns the number of channels
    pub fn num_channels(&self) -> u8 {
        self.num_channels
    }

    /// Returns all channels as a slice
    pub fn channels(&self) -> &[T] {
        &self.channels
    }

    /// Returns all channels as a mutable slice
    pub fn channels_mut(&mut self) -> &mut [T] {
        &mut self.channels
    }

    /// Returns all channels except the last channel as a slice
    pub fn channels_no_alpha(&self) -> &[T] {
        &self.channels[..(self.num_channels as usize)]
    }

    /// Returns the last channel
    pub fn alpha(&self) -> T {
        self.channels[(self.num_channels - 1) as usize]
    }

    /// Applies function `f` to all channels
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

    /// Applies function `f` to all channels except the last (alpha) channel, and applies
    /// function `g` to the alpha channel
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

impl<T: Number> std::fmt::Display for Pixel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_str = "(".to_owned();

        print_str.push_str(&self.channels[0].to_string());
        for i in 1..(self.num_channels as usize) {
            print_str.push_str(", ");
            print_str.push_str(&self.channels[i].to_string());
        }
        print_str.push_str(")");

        write!(f, "{}", print_str.as_str())
    }
}

/// An Image representation
///
/// # Fields
/// * `width` - The width of the image
/// * `height` - The height of the image
/// * `channels` - The number of channels in each `Pixel`
/// * `alpha` - A `bool` that is `true` if the image has an alpha channel, `false` otherwise
/// * `pixels` - A `Vec` of `Pixel`s containing the image data
#[derive(Debug, Clone)]
pub struct Image<T: Number> {
    width: u32,
    height: u32,
    channels: u8,
    alpha: bool,
    pixels: Vec<Pixel<T>>,
}

impl<T: Number> Image<T> {
    /// Creates a new `Image`
    pub fn new(width: u32, height: u32, channels: u8, alpha: bool, data: &[T]) -> Self {
        let mut pixels = Vec::new();
        let size = (width * height * channels as u32) as usize;
        for i in (0..size).step_by(channels as usize) {
            let pixel = Pixel::new(&data[i..(i + channels as usize)]);
            pixels.push(pixel);
        }

        Image { width, height, channels, alpha, pixels }
    }

    /// Creates an `Image` populated with zeroes
    pub fn blank(width: u32, height: u32, channels: u8, alpha: bool) -> Self {
        Image {
            width,
            height,
            channels,
            alpha,
            pixels: vec![Pixel::blank(channels); (width * height) as usize],
        }
    }

    /// Returns the `width` and `height` of `self`
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the `width`, `height`, and `channels` of `self`
    pub fn dimensions_with_channels(&self) -> (u32, u32, u8) {
        (self.width, self.height, self.channels)
    }

    /// Returns `true` if `self` has an alpha channel, `false` otherwise
    pub fn has_alpha(&self) -> bool {
        self.alpha
    }

    /// Returns `pixels` as a slice of `Pixel`s
    pub fn pixels(&self) -> &[Pixel<T>] {
        &self.pixels
    }

    /// Returns `pixels` as `Vec<T>`
    pub fn pixels_as_vector(&self) -> Vec<T> {
        let mut pixels_vec = Vec::new();

        for i in self.pixels.iter() {
            for j in i.channels().iter() {
                pixels_vec.push(*j);
            }
        }

        pixels_vec
    }

    /// Returns `pixels` as a mutable slice
    pub fn pixel_mut(&mut self, x: u32, y: u32) -> &mut [T] {
        self.pixels[(y * self.width + x) as usize].channels_mut()
    }

    /// Returns a reference to the `Pixel` located at `(x, y)`
    pub fn get_pixel(&self, x: u32, y: u32) -> &Pixel<T> {
        &self.pixels[(y * self.width + x) as usize]
    }

    /// Replaces the `Pixel` located at `(x, u)` with `p`
    pub fn put_pixel(&mut self, x: u32, y: u32, p: Pixel<T>) {
        self.pixels[(y * self.width + x) as usize] = p;
    }

    /// Returns a `Vec` of `Pixel` references to the "strip" of `Pixel`s centered at `(x, y)`.
    /// Uses clamp padding for edge pixels (edge pixels are repeated indefinitely)
    ///
    /// # Arguments
    ///
    /// * `size` - The length of the strip of `Pixel`s; must be an odd number
    /// * `is_vert` - If `true`, this function returns vertical strips; if `false`, horizontal strips
    pub fn get_neighborhood_vec(&self, x: u32, y: u32, size: u32, is_vert: bool) -> Vec<&Pixel<T>> {
        let mut vec = vec![self.get_pixel(x, y); size as usize];
        let center = (size/2) as usize;

        // TODO: Make this cleaner
        if is_vert {
            for i in 1..((size as usize) / 2 + 1) {
                if y > i as u32 {
                    vec[center-i] = self.get_pixel(x, y - (i as u32));
                }

                if y + (i as u32) < self.height {
                    vec[center+i] = self.get_pixel(x, y + (i as u32));
                }
            }
        } else {
            for i in 1..(size as usize / 2 + 1) {
                if x > i as u32 {
                    vec[center-i] = self.get_pixel(x - (i as u32), y);
                }

                if x + (i as u32) < self.width {
                    vec[center+i] = self.get_pixel(x + (i as u32), y);
                }

            }
        }

        vec
    }

    /// Returns a `Vec` of `Pixel` references to the "square" of `Pixel`s centered at `(x, y)`.
    /// Uses clamp padding for edge pixels (edge pixels are repeated indefinitely)
    ///
    /// # Arguments
    ///
    /// * `size` - The length/width of the square of `Pixel`s; must be an odd number
    pub fn get_neighborhood_square(&self, x: u32, y: u32, size: u32) -> Vec<&Pixel<T>> {
        let mut vec = Vec::new();

        // Add center pixel
        // TODO

        vec
    }

    /// Applies function `f` to each `Pixel` in `pixels`
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

    /// If `alpha`, applies function `f` to the non-alpha portion of each `Pixel` in `pixels` and
    /// applies function `g` to the alpha channel of each `Pixel` in `pixels`;
    /// otherwise, applies function `f` to each `Pixel` in `pixels`
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

    /// Applies function `f` to each channel of each `Pixel` in `pixels`
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

    /// If `alpha`, applies function `f` to each non-alpha channel of each `Pixel` in `pixels` and
    /// applies function `g` to the alpha channel of each `Pixel` in `pixels`;
    /// otherwise, applies function `f` to each channel of each `Pixel` in `pixels`
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

    /// Applies function `f` to each channel of index `channel_index` of each `Pixel` in `pixels`.
    /// Modifies `self`
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