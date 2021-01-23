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
        &self.channels[..((self.num_channels  as usize)-1)]
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

    /// Applies function `f` to all channels
    pub fn apply<F>(&mut self, f: F)
        where F: Fn(T) -> T {
        for i in 0..(self.num_channels as usize) {
            self.channels[i] = f(self.channels[i]);
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

    /// Applies function `f` to all channels except the last (alpha) channel, and applies
    /// function `g` to the alpha channel
    pub fn apply_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(T) -> T,
              G: Fn(T) -> T {
        for i in 0..((self.num_channels as usize) - 1) {
            self.channels[i] = f(self.channels[i]);
        }

        self.channels[(self.num_channels as usize)-1] = g(self.alpha());
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
/// * `channels` - The number of channels in each pixel of the image
/// * `alpha` - A `bool` that is `true` if the image has an alpha channel, `false` otherwise
/// * `pixels` - A `Vec` containing the image data
#[derive(Debug, Clone)]
pub struct Image<T: Number> {
    width: u32,
    height: u32,
    channels: u32,
    size: u32,
    alpha: bool,
    pixels: Vec<T>,
}

impl<T: Number> Image<T> {
    /// Creates a new `Image<T>`
    pub fn new(width: u32, height: u32, channels: u32, alpha: bool, data: &[T]) -> Self {
        let mut pixels = Vec::new();
        let size = (width * height * channels as u32) as usize;
        for i in 0..size {
            pixels.push(data[i]);
        }

        Image {
            width,
            height,
            channels,
            size: width * height * channels,
            alpha,
            pixels }
    }

    /// Creates an `Image<T>` populated with zeroes
    pub fn blank(width: u32, height: u32, channels: u32, alpha: bool) -> Self {
        Image {
            width,
            height,
            channels,
            size: width * height * channels,
            alpha,
            pixels: vec![0.into(); (width * height * channels as u32) as usize],
        }
    }

    /// Creates an empty `Image<T>`
    pub fn empty(width: u32, height: u32, channels: u32, alpha: bool) -> Self {
        Image {
            width,
            height,
            channels,
            size: width * height * channels,
            alpha,
            pixels: Vec::new(),
        }
    }

    /// Returns the number of channels
    pub fn channels(&self) -> u32 {
        self.channels
    }

    /// Returns the size
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Returns the width and height (in that order)
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the width, height, and channels (in that order)
    pub fn dimensions_with_channels(&self) -> (u32, u32, u32) {
        (self.width, self.height, self.channels)
    }

    /// Returns alpha
    pub fn has_alpha(&self) -> bool {
        self.alpha
    }

    /// Returns `pixels` as a slice
    pub fn pixels(&self) -> &[T] {
        &self.pixels
    }

    /// Returns `pixels` as a mutable slice
    pub fn pixels_mut(&mut self, x: u32, y: u32) -> &mut [T] {
        &mut self.pixels
    }

    /// Returns a slice representing the pixel at `(x, y)`
    pub fn get_pixel(&self, x: u32, y: u32) -> &[T] {
        let index = (y * self.width + x) as usize;
        &self.pixels[index..(index + self.channels as usize)]
    }

    /// Returns a slice representing the pixel at `(x, y)` without the last channel
    pub fn get_pixel_without_alpha(&self, x: u32, y: u32) -> &[T] {
        let index = (y * self.width + x) as usize;
        &self.pixels[index..(index + self.channels as usize - 1)]
    }

    /// Returns the last channel of the pixel at `(x, y)`
    pub fn get_alpha(&self, x: u32, y: u32) -> T {
        let index = (y * self.width + x) as usize;
        self.pixels[index + self.channels as usize]
    }

    /// Returns the channel at index `i`
    pub fn get_channel(&self, i: usize) -> T {
        self.pixels[i]
    }

    /// Replaces the pixel located at `(x, y)` with `p`
    pub fn put_pixel(&mut self, x: u32, y: u32, p: &[T]) {
        if p.len() != self.channels as usize {
            return;
        }

        let index = (y * self.width + x) as usize;
        for i in 0..(self.channels as usize) {
            self.pixels[i + index] = p[i];
        }
    }

    /// Replaces the channel at index `i` with `channel`
    pub fn put_channel(&mut self, i: usize, channel: T) {
        self.pixels[i] = channel;
    }

    // TODO: add dimension checks on get_neighborhood_* methods
    /// Returns a slice representing a row or column of pixels of length `size` centered at `(x, y)`.
    /// If `is_vert` is `true`, returns the column; otherwise, returns the row.
    /// Uses clamp padding for edge pixels (edge pixels are repeated indefinitely)
    pub fn get_neighborhood_1d(&self, x: u32, y: u32, size: u32, is_vert: bool) -> &[T] {
        let mut vec = vec![0.into(); (size * self.channels as u32) as usize];
        let center = (size / 2) as usize;

        if is_vert {
            for i in 1..((size as usize) / 2 + 1) {
                let mut pixel_up = self.get_pixel(x, y);
                let mut pixel_down = self.get_pixel(x, y);

                if y >= i as u32 {
                    pixel_up = self.get_pixel(x, y - (i as u32));
                }

                if y + (i as u32) < self.height {
                    pixel_down = self.get_pixel(x, y + (i as u32));
                }

                for j in 0..(self.channels as usize) {
                    vec[center-i+j] = pixel_up[j];
                    vec[center+i+j] = pixel_down[j];
                }
            }
        } else {
            for i in 1..(size as usize / 2 + 1) {
                let mut pixel_left = self.get_pixel(x, y);
                let mut pixel_right = self.get_pixel(x, y);

                if x >= i as u32 {
                    pixel_left = self.get_pixel(x - (i as u32), y);
                }

                if x + (i as u32) < self.width {
                    pixel_right = self.get_pixel(x + (i as u32), y);
                }

                for j in 0..(self.channels as usize) {
                    vec[center-i+j] = pixel_left[j];
                    vec[center+i+j] = pixel_right[j];
                }
            }
        }

        &vec
    }

    /// Returns a slice representing the "square" of pixels of side length `size` centered at `(x, y)`.
    /// Uses clamp padding for edge pixels (edge pixels are repeated indefinitely)
    pub fn get_neighborhood_2d(&self, x: u32, y: u32, size: u32) -> &[T] {
        let mut vec = Vec::new();
        let start_x = (x as i32) - (size as i32) / 2;
        let start_y = (y as i32) - (size as i32) / 2;

        for i in 0..size {
            for j in 0..size {
                let mut curr_x = start_x + (j as i32);
                let mut curr_y = start_y + (i as i32);

                if curr_x < 0 || curr_x >= self.width as i32 { curr_x = x as i32 };
                if curr_y < 0 || curr_y >= self.height as i32 { curr_y = y as i32 };

                vec.extend_from_slice(self.get_pixel(curr_x as u32, curr_y as u32));
            }
        }

        &vec
    }

    /// Applies function `f` to each pixel
    pub fn map_pixels<S: Number, F>(&self, f: F) -> Image<S>
        where F: Fn(&[T]) -> &[S] {
        let (width, height) = self.dimensions();
        let mut pixels = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let p = f(self.get_pixel(x, y));
                pixels.extend_from_slice(p);
            }
        }

        let channels = pixels.len() as u32 / (width * height);

        Image {
            width,
            height,
            channels,
            size: width * height * channels,
            alpha: self.alpha,
            pixels,
        }
    }

    /// Applies function `f` to pixel
    pub fn apply_pixels<F>(&mut self, f: F)
        where F: Fn(&[T]) -> &[T] {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, f(self.get_pixel(x, y)));
            }
        }
    }

    /// If `alpha`, applies function `f` to the non-alpha portion of each pixel and
    /// applies function `g` to the alpha channel of each pixel;
    /// otherwise, applies function `f` to each pixel
    pub fn map_pixels_if_alpha<S: Number, F, G>(&self, f: F, g: G) -> Image<S>
        where F: Fn(&[T]) -> &[S],
              G: Fn(T) -> S {
        let (width, height) = self.dimensions();
        let mut pixels = Vec::new();

        if !self.alpha {
            return self.map_pixels(f);
        }

        for y in 0..height {
            for x in 0..width {
                let mut v = f(self.get_pixel_without_alpha(x, y)).to_vec();
                v.push(g(self.get_alpha(x, y)));
                pixels.append(&mut v);
            }
        }

        let channels = pixels.len() as u32 / (width * height);

        Image {
            width,
            height,
            channels,
            size: width * height * channels,
            alpha: self.alpha,
            pixels,
        }
    }

    /// If `alpha`, applies function `f` to the non-alpha portion of each pixel and
    /// applies function `g` to the alpha channel of each pixel;
    /// otherwise, applies function `f` to each pixel
    pub fn apply_pixels_if_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(&[T]) -> &[T],
              G: Fn(T) -> T {
        if !self.alpha {
            self.apply_pixels(f);
            return;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let mut v = f(self.get_pixel_without_alpha(x, y)).to_vec();
                v.push(g(self.get_alpha(x, y)));
                self.put_pixel(x, y,&v);
            }
        }
    }

    /// Applies function `f` to each channel of pixel
    pub fn map_channels<S: Number, F>(&self, f: F) -> Image<S>
        where F: Fn(T) -> S {
        let mut pixels = Vec::new();

        for i in 0..(self.size as usize) {
            pixels.push(f(self.pixels[i]));
        }

        Image {
            width: self.width,
            height: self.height,
            channels: self.channels,
            size: self.size,
            alpha: self.alpha,
            pixels,
        }
    }

    /// Applies function `f` to each channel of each pixel
    pub fn apply_channels<F>(&mut self, f: F)
        where F: Fn(T) -> T {
        for i in 0..(self.size as usize) {
            self.pixels[i] = f(self.pixels[i]);
        }
    }

    /// If `alpha`, applies function `f` to each non-alpha channel of each pixel and
    /// applies function `g` to the alpha channel of each pixel;
    /// otherwise, applies function `f` to each channel of each pixel
    pub fn map_channels_if_alpha<S: Number, F, G>(&self, f: F, g: G) -> Image<S>
        where F: Fn(T) -> S,
              G: Fn(T) -> S {
        if !self.alpha {
            return self.map_channels(f);
        }

        let mut pixels = Vec::new();
        for i in (0..(self.size as usize)).step_by(self.channels as usize) {
            for j in 0..(self.channels as usize - 1) {
                pixels.push(f(self.pixels[i+j]));
            }

            pixels.push(g(self.pixels[i+(self.channels as usize)-1]));
        }

        Image {
            width: self.width,
            height: self.height,
            channels: self.channels,
            size: self.size,
            alpha: self.alpha,
            pixels,
        }
    }

    /// If `alpha`, applies function `f` to each non-alpha channel of each `Pixel` in `pixels` and
    /// applies function `g` to the alpha channel of each `Pixel` in `pixels`;
    /// otherwise, applies function `f` to each channel of each `Pixel` in `pixels`
    pub fn apply_channels_if_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(T) -> T,
              G: Fn(T) -> T {
        if !self.alpha {
            self.apply_channels(f);
            return;
        }

        for i in (0..(self.size as usize)).step_by(self.channels as usize) {
            for j in 0..(self.channels as usize - 1) {
                self.pixels[i+j] = f(self.pixels[i+j]);
            }

            self.pixels[i+(self.channels as usize)-1] = g(self.pixels[i+(self.channels as usize)-1]);
        }

    }

    /// Applies function `f` to each channel of index `channel_index` of each pixel.
    /// Modifies `self`
    pub fn edit_channel<F>(&mut self, f: F, channel_index: usize)
        where F: Fn(T) -> T {
        let (width, height) = self.dimensions();

        for i in (0..(self.size as usize)).step_by(self.channels as usize) {
            self.pixels[i+channel_index] = f(self.pixels[i+channel_index]);
        }
    }
}