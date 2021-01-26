use imgproc_rs::image::{Image, ImageInfo, SubImage, BaseImage, Pixel};

#[test]
fn image_general_test() {
    let mut img_new: Image<u8> = Image::new(3, 3, 4, true,
                             &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 10, 3, 4, 5, 6]);
    let img_blank: Image<f64> = Image::blank(ImageInfo::new(3, 3, 3, false));

    let pixels_new = &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                    1, 2, 3, 4, 2, 3, 4, 10, 3, 4, 5, 6];
    let pixels_blank: Vec<f64> = vec![0.0; 27];

    // Test ImageInfo
    let info = ImageInfo{width: 3, height: 3, channels: 4, alpha: true};
    assert_eq!(info, img_new.info());

    // Test getter methods
    assert_eq!([2, 3, 4, 10], img_new.get_pixel(1, 2));
    assert_eq!([2, 3, 4, 5], img_new[1]);
    assert_eq!(10, img_new[7][3]);
    assert_eq!(4, img_new.get_pixel(0, 0).alpha());
    assert_eq!([3, 4, 5], img_new.get_pixel(2, 2).channels_without_alpha());
    assert_eq!(pixels_new, img_new.data());
    assert_eq!(pixels_blank, img_blank.data());

    // Test setter methods
    let pixel: [u8; 4] = [1, 1, 1, 1];
    img_new.set_pixel(2, 2, &pixel);
    assert_eq!(pixel, img_new.get_pixel(2, 2,));
    img_new.set_pixel_indexed(3, &pixel);
    assert_eq!(pixel, img_new[3]);
}

#[test]
fn image_get_subimage_test() {
    let img: Image<u8> = Image::new(3, 3, 4, true,
                                        &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                                                6, 5, 4, 3, 5, 4, 3, 2, 4, 3, 2, 1,
                                                2, 4, 6, 8, 3, 5, 7, 9, 1, 3, 5, 7]);

    // Test get_subimage()
    assert_eq!(vec![&[6, 5, 4, 3], &[2, 4, 6, 8]],
               img.get_subimage(0, 1, 1, 2).data());

    // Test get_neighborhood_1d()
    assert_eq!(vec![&[2, 4, 6, 8],
                    &[3, 5, 7, 9],
                    &[1, 3, 5, 7]],
               img.get_neighborhood_1d(1, 2, 3, false).data());
    assert_eq!(vec![&[3, 4, 5, 6],
                    &[4, 3, 2, 1],
                    &[1, 3, 5, 7]],
               img.get_neighborhood_1d(2, 1, 3, true).data());
    assert_eq!(vec![&[1, 2, 3, 4],
                    &[1, 2, 3, 4],
                    &[2, 3, 4, 5]],
               img.get_neighborhood_1d(0, 0, 3, false).data());
    assert_eq!(vec![&[5, 4, 3, 2],
                    &[3, 5, 7, 9],
                    &[3, 5, 7, 9]],
               img.get_neighborhood_1d(1, 2, 3, true).data());

    // Test get_neighborhood_2d()
    assert_eq!(vec![&[1, 2, 3, 4],
                    &[2, 3, 4, 5],
                    &[3, 4, 5, 6],
                    &[6, 5, 4, 3],
                    &[5, 4, 3, 2],
                    &[4, 3, 2, 1],
                    &[2, 4, 6, 8],
                    &[3, 5, 7, 9],
                    &[1, 3, 5, 7]],
               img.get_neighborhood_2d(1, 1, 3).data());
    assert_eq!(vec![&[1, 2, 3, 4],
                    &[1, 2, 3, 4],
                    &[2, 3, 4, 5],
                    &[1, 2, 3, 4],
                    &[1, 2, 3, 4],
                    &[2, 3, 4, 5],
                    &[6, 5, 4, 3],
                    &[6, 5, 4, 3],
                    &[5, 4, 3, 2]],
               img.get_neighborhood_2d(0, 0, 3).data());
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
    assert_eq!(&[6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map1.data());

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
    assert_eq!(&[6, 7, 8, 4, 7, 8, 9, 5, 11, 10, 9, 3, 10, 9, 8, 2], map2.data());
    assert_eq!(&[6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map3.data());

    // Test map_channels()
    let map4 = img1.map_channels(|channel| channel + 5);
    assert_eq!(&[6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map4.data());

    // Test map_channels_if_alpha()
    let map5 = img1.map_channels_if_alpha(|channel| channel + 5, |a| a);
    let map6 = img2.map_channels_if_alpha(|channel| channel + 5, |a| a);
    assert_eq!(&[6, 7, 8, 4, 7, 8, 9, 5, 11, 10, 9, 3, 10, 9, 8, 2], map5.data());
    assert_eq!(&[6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map6.data());

    // Test edit_channel()
    img1.edit_channel(|channel| channel + 5, 2);
    assert_eq!(&[1, 2, 8, 4, 2, 3, 9, 5, 6, 5, 9, 3, 5, 4, 8, 2], img1.data());
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
        vec
    });
    assert_eq!(&[6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], img1.data());

    // Test apply_pixels_if_alpha()
    img1.apply_pixels_if_alpha(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        vec
    }, |a| a);
    img2.apply_pixels_if_alpha(|channels| {
        let mut vec = Vec::new();
        for channel in channels.iter() {
            vec.push(channel + 5);
        }
        vec
    }, |a| a);
    assert_eq!(&[11, 12, 13, 9, 12, 13, 14, 10, 16, 15, 14, 8, 15, 14, 13, 7], img1.data());
    assert_eq!(&[6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], img2.data());

    // Test apply_channels()
    img1.apply_channels(|channel| channel + 5);
    assert_eq!(&[16, 17, 18, 14, 17, 18, 19, 15, 21, 20, 19, 13, 20, 19, 18, 12], img1.data());

    // Test apply_channels_if_alpha()
    img1.apply_channels_if_alpha(|channel| channel - 5, |a| a);
    img2.apply_channels_if_alpha(|channel| channel - 5, |a| a);
    assert_eq!(&[11, 12, 13, 14, 12, 13, 14, 15, 16, 15, 14, 13, 15, 14, 13, 12], img1.data());
    assert_eq!(&[1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2], img2.data());
}

#[test]
fn subimage_test() {
    let subimg = SubImage::new(2, 2, 3, false,
                               vec![&[1, 2, 3], &[4, 5, 6], &[6, 5, 4], &[3, 2, 1]]);

    let pixel = [6, 5, 4];
    assert_eq!(pixel, subimg[2]);
    assert_eq!(pixel, subimg.get_pixel(0, 1));
}
