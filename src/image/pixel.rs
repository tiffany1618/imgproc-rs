use crate::image::base::Number;

/// A trait for image pixels
pub trait Pixel<T: Number> {
    /// Returns the last channel of the pixel
    fn alpha(&self) -> T;

    /// Returns the last channel of the pixel
    fn channels_without_alpha(&self) -> &[T];

    /// Applies function `f` to each channel
    fn map_all<S: Number, F>(&self, f: F) -> Vec<S>
        where F: Fn(T) -> S;

    /// Applies function `f` to each channel except the last channel, and applies
    /// function `g` to the alpha channel
    fn map_alpha<S: Number, F, G>(&self, f: F, g: G) -> Vec<S>
        where F: Fn(T) -> S,
              G: Fn(T) -> S;

    /// Applies function `f` to each channel
    fn apply<F>(&mut self, f: F)
        where F: Fn(T) -> T;

    /// Applies function `f` to each channel except the last channel, and applies
    /// function `g` to the alpha channel
    fn apply_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(T) -> T,
              G: Fn(T) -> T;
}

impl<T: Number> Pixel<T> for [T] {
    fn alpha(&self) -> T {
        self[self.len()-1]
    }

    fn channels_without_alpha(&self) -> &[T] {
        &self[..(self.len()-1)]
    }

    fn map_all<S: Number, F>(&self, f: F) -> Vec<S>
        where F: Fn(T) -> S {
        let mut channels_out = Vec::new();

        for channel in self.iter() {
            channels_out.push(f(*channel));
        }

        channels_out
    }

    fn map_alpha<S: Number, F, G>(&self, f: F, g: G) -> Vec<S>
        where F: Fn(T) -> S,
              G: Fn(T) -> S {
        let mut channels_out = Vec::new();

        for channel in self.channels_without_alpha().iter() {
            channels_out.push(f(*channel));
        }

        channels_out.push(g(self.alpha()));
        channels_out
    }

    fn apply<F>(&mut self, f: F)
        where F: Fn(T) -> T {
        for i in 0..self.len() {
            self[i] = f(self[i]);
        }
    }

    fn apply_alpha<F, G>(&mut self, f: F, g: G)
        where F: Fn(T) -> T,
              G: Fn(T) -> T {
        for i in 0..(self.len() - 1) {
            self[i] = f(self[i]);
        }

        self[self.len()-1] = g(self.alpha());
    }
}