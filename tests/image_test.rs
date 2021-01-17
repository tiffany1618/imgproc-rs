use imgproc_rs::image::{Image, Pixel};

#[test]
fn pixel_test() {
    let pixel_new: Pixel<u8> = Pixel::new(&vec![1, 2, 3]);
    let pixel_blank: Pixel<f64> = Pixel::blank(3);

    // Test basic methods
    assert_eq!(3, pixel_new.num_channels());
    assert_eq!(3, pixel_blank.num_channels());
    assert_eq!(vec![1, 2, 3], pixel_new.channels());
    assert_eq!(vec![0.0, 0.0, 0.0], pixel_blank.channels());
    assert_eq!(vec![1, 2], pixel_new.channels_no_alpha());
    assert_eq!(vec![0.0, 0.0], pixel_blank.channels_no_alpha());
    assert_eq!(3, pixel_new.alpha());
    assert_eq!(0.0, pixel_blank.alpha());
}

#[test]
fn pixel_map_test() {
    let pixel: Pixel<u8> = Pixel::new(&vec![1, 2, 3]);
    let mapped_pixel = pixel.map(|channel| channel + 5);

    assert_eq!(vec![6, 7, 8], mapped_pixel.channels());
}

#[test]
fn pixel_map_alpha_test() {
    let pixel: Pixel<u8> = Pixel::new(&vec![1, 2, 3]);
    let mapped_pixel = pixel.map_alpha(|channel| channel + 5, |a| a);

    assert_eq!(vec![6, 7, 3], mapped_pixel.channels());
}

#[test]
fn image_test() {
    let img_new: Image<u8> = Image::new(3, 3, 4, true,
                             &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6]);
    let img_blank: Image<f64> = Image::blank(3, 3, 3, false);

    let pixels_new = vec![Pixel::new(&[1, 2, 3, 4]), Pixel::new(&[2, 3, 4, 5]), Pixel::new(&[3, 4, 5, 6]),
                          Pixel::new(&[1, 2, 3, 4]), Pixel::new(&[2, 3, 4, 5]), Pixel::new(&[3, 4, 5, 6]),
                          Pixel::new(&[1, 2, 3, 4]), Pixel::new(&[2, 3, 4, 5]), Pixel::new(&[3, 4, 5, 6])];
    let pixels_blank: Vec<Pixel<f64>> = vec![Pixel::blank(3); 9];

    // Test basic methods
    assert_eq!((3, 3), img_new.dimensions());
    assert_eq!((3, 3), img_blank.dimensions());
    assert_eq!((3, 3, 4), img_new.dimensions_with_channels());
    assert_eq!((3, 3, 3), img_blank.dimensions_with_channels());
    assert_eq!(true, img_new.has_alpha());
    assert_eq!(false, img_blank.has_alpha());
    assert_eq!(&pixels_new, img_new.pixels());
    assert_eq!(&pixels_blank, img_blank.pixels());
    assert_eq!(vec![1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                 1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                 1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6], img_new.pixels_as_vector());
    assert_eq!(vec![0.0; 27], img_blank.pixels_as_vector());
}