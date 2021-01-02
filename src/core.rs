use crate::util::Number;
use crate::image::{Image, Pixel};

// TODO: Fix loss of precision by integer division
pub fn rgb_to_grayscale<T: Number>(input: &Image<T>) -> Image<T> {
    input.map_pixels(|channels_in| {
        vec![(channels_in[0] / 3.into()) + (channels_in[1] / 3.into()) + (channels_in[2] / 3.into())]
    }, false)
}