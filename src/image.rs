
// TODO: add primitive type constraint on Pixel and Image

enum PixelType {
    // TODO
}

pub struct Pixel<T> {
    num_channels: u8,
    channels: Vec<T>,
}

impl<T> Pixel<T> {
    pub fn new<S>(channels: Vec<S>) -> Pixel<S> {
        Pixel {
            num_channels: channels.len(),
            channels,
        }
    }

    pub fn empty<S>(num_channels: u8) -> Pixel<S> {
        Pixel {
            num_channels,
            channels: vec![0; num_channels as usize],
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
        self.channels[0..self.num_channels]
    }

    // Apply function f to all channels
    pub fn map<S>(&self, f: fn(T) -> S) -> Pixel<S> {
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
    pub fn map_no_alpha<S>(&self, f: fn(T) -> S) -> Pixel<S> {
        let mut channels = Vec::new();

        for i in 0..(self.num_channels - 1) {
            channels.push(f(self.channels[0]));
        }

        channels.push(self.alpha());

        Pixel {
            num_channels: self.num_channels,
            channels,
        }
    }
}

pub struct Image<T> {
    width: u32,
    height: u32,
    channels: u8,
    data: Vec<Pixel<T>>,
}

impl<T> Image<T> {
    pub fn new<S>(path: &str) -> Image<S> {
        let img = image::open(path).unwrap();
        let (width, height) = img.dimensions();
        let channels = img.get_pixel(0, 0).len();
        let data = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let pixel = Pixel::new(img.get_pixels(x, y).channels());
                data.push(pixel);
            }
        }

        Image {
            width,
            height,
            channels,
            data,
        }
    }

    pub fn empty<S>(width: u32, height: u32, channels: u8) -> Image<S> {
        Image {
            width,
            height,
            channels,
            data: vec![Pixel::empty(channels); (width * height) as usize],
        }
    }

    pub fn dimensions(&self) -> (u32, u32, u8) {
        (self.width, self.height, self.channels)
    }

    pub fn data(&self) -> Vec<Pixel<T>> {
        self.data
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Pixel<T>{
        self.data[y * self.width + x]
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, p: Pixel<T>) {
        self.data[y * self.width + x] = p;
    }

    pub fn write(&self, path: &str) {
        let (width, height) = self.dimensions();
        let img = image::ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, image::Rgba(self.get_pixel(x, y).channels()));
            }
        }

        img.save(path);
    }
}