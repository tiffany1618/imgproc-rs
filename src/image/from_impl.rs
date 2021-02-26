use crate::image::Image;

impl From<Image<u8>> for Image<f64> {
    fn from(img: Image<u8>) -> Image<f64> {
        img.map_channels(|channel| channel as f64)
    }
}

impl From<Image<u16>> for Image<f64> {
    fn from(img: Image<u16>) -> Image<f64> {
        img.map_channels(|channel| channel as f64)
    }
}

impl From<Image<u32>> for Image<f64> {
    fn from(img: Image<u32>) -> Image<f64> {
        img.map_channels(|channel| channel as f64)
    }
}

impl From<Image<u64>> for Image<f64> {
    fn from(img: Image<u64>) -> Image<f64> {
        img.map_channels(|channel| channel as f64)
    }
}

impl From<Image<u128>> for Image<f64> {
    fn from(img: Image<u128>) -> Image<f64> {
        img.map_channels(|channel| channel as f64)
    }
}

impl From<Image<usize>> for Image<f64> {
    fn from(img: Image<usize>) -> Image<f64> {
        img.map_channels(|channel| channel as f64)
    }
}

impl From<Image<f32>> for Image<f64> {
    fn from(img: Image<f32>) -> Image<f64> {
        img.map_channels(|channel| channel as f64)
    }
}

impl From<Image<f64>> for Image<u8> {
    fn from(img: Image<f64>) -> Image<u8> {
        img.map_channels(|channel| channel.round() as u8)
    }
}

impl From<Image<f32>> for Image<u8> {
    fn from(img: Image<f32>) -> Image<u8> {
        img.map_channels(|channel| channel.round() as u8)
    }
}