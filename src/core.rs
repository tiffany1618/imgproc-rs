use crate::util::Primitive;
use crate::image::Image;

pub fn rgb_to_grayscale<T: Primitive>(input: &Image<T>) -> Image<T> {
    input.map_pixels(|channels_in| {
        ((channels_in[0] + channels_in[1] + channels_in[2]) / 3 as T).round() as T
    }, false)
}