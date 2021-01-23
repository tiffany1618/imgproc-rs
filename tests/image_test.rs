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
fn pixel_apply_test() {
    let mut pixel: Pixel<u8> = Pixel::new(&[1, 2, 3]);

    // Test apply()
    pixel.apply(|channel| channel + 5);
    assert_eq!(&[6, 7, 8], pixel.channels());

    // Test apply_alpha()
    pixel.apply_alpha(|channel| channel + 5, |a| a);
    assert_eq!(&[11, 12, 8], pixel.channels());
}

#[test]
fn image_test() {
    let mut img_new: Image<u8> = Image::new(3, 3, 4, true,
                             &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 10, 3, 4, 5, 6]);
    let img_blank: Image<f64> = Image::blank(3, 3, 3, false);

    let pixels_new = &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 10, 3, 4, 5, 6];
    let pixels_blank: Vec<f64> = vec![0.0; 27];

    // Test basic methods
    assert_eq!(4, img_new.channels());
    assert_eq!(3, img_blank.channels());
    assert_eq!(3 * 3 * 4, img_new.size());
    assert_eq!(3 * 3 * 3, img_blank.size());
    assert_eq!((3, 3), img_new.dimensions());
    assert_eq!((3, 3), img_blank.dimensions());
    assert_eq!((3, 3, 4), img_new.dimensions_with_channels());
    assert_eq!((3, 3, 3), img_blank.dimensions_with_channels());
    assert_eq!(true, img_new.has_alpha());
    assert_eq!(false, img_blank.has_alpha());
    assert_eq!(pixels_new, img_new.pixels());
    assert_eq!(pixels_blank, img_blank.pixels());

    // Test pixel get/put methods
    assert_eq!(&[2, 3, 4, 10], img_new.get_pixel(1, 2));
    assert_eq!(&[2, 3, 4], img_new.get_pixel_without_alpha(1, 2));
    assert_eq!(10, img_new.get_alpha(1, 2));

    img_new.put_pixel(1, 1, &[9, 8, 7, 6]);
    assert_eq!(&[9, 8, 7, 6], img_new.get_pixel(1, 1));
    assert_eq!(&[9, 8, 7], img_new.get_pixel_without_alpha(1, 2));
    assert_eq!(6, img_new.get_alpha(1, 2));

    // Test channel get/put methods
    assert_eq!(10, img_new.get_channel(3));

    img_new.put_channel(3, 11);
    assert_eq!(11, img_new.get_channel(3));
}

#[test]
fn image_get_neighborhood_test() {
    let img: Image<u8> = Image::new(3, 3, 4, true,
                                        &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                                6, 5, 4, 3, 5, 4, 3, 2, 4, 3, 2, 1,
                                                2, 4, 6, 8, 3, 5, 7, 9, 1, 3, 5, 7]);

    // Test get_neighborhood_vec()
    assert_eq!(&[2, 4, 6, 8,
                 3, 5, 7, 9,
                 1, 3, 5, 7],
               img.get_neighborhood_1d(1, 2, 3, false));
    assert_eq!(&[3, 4, 5, 6,
                 4, 3, 2, 1,
                 1, 3, 5, 7],
               img.get_neighborhood_1d(2, 1, 3, true));
    assert_eq!(&[1, 2, 3, 4,
                    1, 2, 3, 4,
                    2, 3, 4, 5],
               img.get_neighborhood_1d(0, 0, 3, false));
    assert_eq!(&[5, 4, 3, 2,
                    3, 5, 7, 9,
                    3, 5, 7, 9],
               img.get_neighborhood_1d(1, 2, 3, true));

    // Test get_neighborhood_square()
    assert_eq!(&[1, 2, 3, 4,
                    2, 3, 4, 5,
                    3, 4, 5, 6,
                    6, 5, 4, 3,
                    5, 4, 3, 2,
                    4, 3, 2, 1,
                    2, 4, 6, 8,
                    3, 5, 7, 9,
                    1, 3, 5, 7],
               img.get_neighborhood_2d(1, 1, 3));
    assert_eq!(&[1, 2, 3, 4,
                    1, 2, 3, 4,
                    2, 3, 4, 5,
                    1, 2, 3, 4,
                    1, 2, 3, 4,
                    2, 3, 4, 5,
                    6, 5, 4, 3,
                    6, 5, 4, 3,
                    5, 4, 3, 2],
               img.get_neighborhood_2d(0, 0, 3));
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
        &vec
    });
    assert_eq!(vec![6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map1.pixels_as_vector());

    // Test map_pixels_if_alpha()
    let map2 = img1.map_pixels_if_alpha(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        &vec
    }, |a| a);
    let map3 = img2.map_pixels_if_alpha(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        &vec
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

#[test]
fn image_apply_test() {
    let mut img1: Image<u8> = Image::new(2, 2, 4, true,
                                         &[1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2]);
    let mut img2: Image<u8> = Image::new(2, 2, 4, false,
                                     &[1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2]);

    // Test apply_pixels()
    img1.apply_pixels(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        &vec
    });
    assert_eq!(vec![6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], img1.pixels_as_vector());

    // Test apply_pixels_if_alpha()
    img1.apply_pixels_if_alpha(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        &vec
    }, |a| a);
    img2.apply_pixels_if_alpha(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        &vec
    }, |a| a);
    assert_eq!(vec![11, 12, 13, 9, 12, 13, 14, 10, 16, 15, 14, 8, 15, 14, 13, 7], img1.pixels_as_vector());
    assert_eq!(vec![6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], img2.pixels_as_vector());

    // Test apply_channels()
    img1.apply_channels(|channel| channel + 5);
    assert_eq!(vec![16, 17, 18, 14, 17, 18, 19, 15, 21, 20, 19, 13, 20, 19, 18, 12], img1.pixels_as_vector());

    // Test apply_channels_if_alpha()
    img1.apply_channels_if_alpha(|channel| channel - 5, |a| a);
    img2.apply_channels_if_alpha(|channel| channel - 5, |a| a);
    assert_eq!(vec![11, 12, 13, 14, 12, 13, 14, 15, 16, 15, 14, 13, 15, 14, 13, 12], img1.pixels_as_vector());
    assert_eq!(vec![1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2], img2.pixels_as_vector());
}