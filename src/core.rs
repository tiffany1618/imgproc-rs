use image::Image;

pub fn rgb_to_grayscale<T>(input: &Image<T>) -> Image<T> {
    let (width, height, channels) = input.dimensions();
    let mut output = Image::empty(width, height, channels);

    for y in 0..height {
        for x in 0..width {
            let p = input.get_pixel(x, y);
            output.put_pixel(x, y, vec![(p[0] + p[1] + p[2]) / 3.0, p.alpha()]);
        }
    }

    output
}
