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

    // Test map()
    let mapped_pixel1 = pixel.map(|channel| channel + 5);
    assert_eq!(vec![6, 7, 8], mapped_pixel1.channels());

    // Test map_alpha()
    let mapped_pixel2 = pixel.map_alpha(|channel| channel + 5, |a| a);
    assert_eq!(vec![6, 7, 3], mapped_pixel2.channels());
}

#[test]
fn image_test() {
    let mut img_new: Image<u8> = Image::new(3, 3, 4, true,
                             &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 10, 3, 4, 5, 6]);
    let img_blank: Image<f64> = Image::blank(3, 3, 3, false);

    let pixels_new: Vec<Pixel<u8>> = vec![Pixel::new(&[1, 2, 3, 4]), Pixel::new(&[2, 3, 4, 5]), Pixel::new(&[3, 4, 5, 6]),
                          Pixel::new(&[1, 2, 3, 4]), Pixel::new(&[2, 3, 4, 5]), Pixel::new(&[3, 4, 5, 6]),
                          Pixel::new(&[1, 2, 3, 4]), Pixel::new(&[2, 3, 4, 10]), Pixel::new(&[3, 4, 5, 6])];
    let pixels_blank: Vec<Pixel<f64>> = vec![Pixel::blank(3); 9];

    // Test basic methods
    assert_eq!((3, 3), img_new.dimensions());
    assert_eq!((3, 3), img_blank.dimensions());
    assert_eq!((3, 3, 4), img_new.dimensions_with_channels());
    assert_eq!((3, 3, 3), img_blank.dimensions_with_channels());
    assert_eq!(true, img_new.has_alpha());
    assert_eq!(false, img_blank.has_alpha());
    assert_eq!(pixels_new, img_new.pixels());
    assert_eq!(pixels_blank, img_blank.pixels());
    assert_eq!(vec![1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                 1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                 1, 2, 3, 4, 2, 3, 4, 10, 3, 4, 5, 6], img_new.pixels_as_vector());
    assert_eq!(vec![0.0; 27], img_blank.pixels_as_vector());

    // Test pixel get/put methods
    assert_eq!(&Pixel::new(&[2, 3, 4, 10]), img_new.get_pixel(1, 2));
    img_new.put_pixel(1, 1, Pixel::new(&[9, 8, 7, 6]));
    assert_eq!(&Pixel::new(&[9, 8, 7, 6]), img_new.get_pixel(1, 1));
}

#[test]
fn image_get_neighborhood_test() {
    let img: Image<u8> = Image::new(3, 3, 4, true,
                                        &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                                6, 5, 4, 3, 5, 4, 3, 2, 4, 3, 2, 1,
                                                2, 4, 6, 8, 3, 5, 7, 9, 1, 3, 5, 7]);

    // Test get_neighborhood_vec()
    assert_eq!(vec![&Pixel::new(&[2, 4, 6, 8]),
                    &Pixel::new(&[3, 5, 7, 9]),
                    &Pixel::new(&[1, 3, 5, 7])],
               img.get_neighborhood_vec(1, 2, 3, false));
    assert_eq!(vec![&Pixel::new(&[3, 4, 5, 6]),
                    &Pixel::new(&[4, 3, 2, 1]),
                    &Pixel::new(&[1, 3, 5, 7])],
               img.get_neighborhood_vec(2, 1, 3, true));
    assert_eq!(vec![&Pixel::new(&[1, 2, 3, 4]),
                    &Pixel::new(&[1, 2, 3, 4]),
                    &Pixel::new(&[2, 3, 4, 5])],
               img.get_neighborhood_vec(0, 0, 3, false));
    assert_eq!(vec![&Pixel::new(&[5, 4, 3, 2]),
                    &Pixel::new(&[3, 5, 7, 9]),
                    &Pixel::new(&[3, 5, 7, 9])],
               img.get_neighborhood_vec(1, 2, 3, true));

    // Test get_neighborhood_square()
    assert_eq!(vec![&Pixel::new(&[1, 2, 3, 4]),
                    &Pixel::new(&[2, 3, 4, 5]),
                    &Pixel::new(&[3, 4, 5, 6]),
                    &Pixel::new(&[6, 5, 4, 3]),
                    &Pixel::new(&[5, 4, 3, 2]),
                    &Pixel::new(&[4, 3, 2, 1]),
                    &Pixel::new(&[2, 4, 6, 8]),
                    &Pixel::new(&[3, 5, 7, 9]),
                    &Pixel::new(&[1, 3, 5, 7])],
               img.get_neighborhood_square(1, 1, 3));
    assert_eq!(vec![&Pixel::new(&[1, 2, 3, 4]),
                    &Pixel::new(&[1, 2, 3, 4]),
                    &Pixel::new(&[2, 3, 4, 5]),
                    &Pixel::new(&[1, 2, 3, 4]),
                    &Pixel::new(&[1, 2, 3, 4]),
                    &Pixel::new(&[2, 3, 4, 5]),
                    &Pixel::new(&[6, 5, 4, 3]),
                    &Pixel::new(&[6, 5, 4, 3]),
                    &Pixel::new(&[5, 4, 3, 2])],
               img.get_neighborhood_square(0, 0, 3));
}

#[test]
fn image_map_test() {
    let mut img1: Image<u8> = Image::new(2, 2, 4, true,
                                    &[1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2]);
    let img2: Image<u8> = Image::new(2, 2, 4, false,
                                     &[1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2]);

    // Test map_pixels()
    let map1 = img1.map_pixels(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        vec
    });
    assert_eq!(vec![6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map1.pixels_as_vector());

    // Test map_pixels_if_alpha()
    let map2 = img1.map_pixels_if_alpha(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        vec
    }, |a| a);
    let map3 = img2.map_pixels_if_alpha(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        vec
    }, |a| a);
    assert_eq!(vec![6, 7, 8, 4, 7, 8, 9, 5, 11, 10, 9, 3, 10, 9, 8, 2], map2.pixels_as_vector());
    assert_eq!(vec![6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map3.pixels_as_vector());

    // Test map_channels()
    let map4 = img1.map_channels(|channel| channel + 5);
    assert_eq!(vec![6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map4.pixels_as_vector());

    // Test map_channels_if_alpha()
    let map5 = img1.map_channels_if_alpha(|channel| channel + 5, |a| a);
    let map6 = img2.map_channels_if_alpha(|channel| channel + 5, |a| a);
    assert_eq!(vec![6, 7, 8, 4, 7, 8, 9, 5, 11, 10, 9, 3, 10, 9, 8, 2], map5.pixels_as_vector());
    assert_eq!(vec![6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map6.pixels_as_vector());

    // Test edit_channel()
    img1.edit_channel(|channel| channel + 5, 2);
    assert_eq!(vec![1, 2, 8, 4, 2, 3, 9, 5, 6, 5, 9, 3, 5, 4, 8, 2], img1.pixels_as_vector());
}