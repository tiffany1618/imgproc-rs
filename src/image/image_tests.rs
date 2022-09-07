#[cfg(test)]
mod image_info_tests {
    use crate::image::ImageInfo;

    fn setup() -> (ImageInfo, ImageInfo) {
        let info1 = ImageInfo::new(10, 10, 4, true);
        let info2 = ImageInfo::new(10, 10, 3, false);

        (info1, info2)
    }

    #[test]
    fn new_test() {
        let (info1, info2) = setup();

        assert_eq!(info1.width, 10);
        assert_eq!(info1.height, 10);
        assert_eq!(info1.channels, 4);
        assert_eq!(info1.alpha, true);

        assert_eq!(info2.width, 10);
        assert_eq!(info2.height, 10);
        assert_eq!(info2.channels, 3);
        assert_eq!(info2.alpha, false);
    }

    #[test]
    fn dimensions_test() {
        let (info1, info2) = setup();

        assert_eq!(info1.wh(), (10, 10));
        assert_eq!(info2.wh(), (10, 10));

        assert_eq!(info1.whc(), (10, 10, 4));
        assert_eq!(info2.whc(), (10, 10, 3));

        assert_eq!(info1.whca(), (10, 10, 4, true));
        assert_eq!(info2.whca(), (10, 10, 3, false));

        assert_eq!(info1.channels_non_alpha(), 3);
        assert_eq!(info2.channels_non_alpha(), 3);
    }

    #[test]
    fn size_test() {
        let (info1, info2) = setup();

        assert_eq!(info1.size(), 10 * 10);
        assert_eq!(info2.size(), 10 * 10);

        assert_eq!(info1.full_size(), 10 * 10 * 4);
        assert_eq!(info2.full_size(), 10 * 10 * 3);
    }
}

#[cfg(test)]
mod image_tests {
    use crate::image::{BaseImage, Image, ImageInfo};

    fn setup() -> Image<i32> {
        let pixels =
            vec![1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                 6, 5, 4, 3, 5, 4, 3, 2, 4, 3, 2, 1,
                 2, 4, 6, 8, 3, 5, 7, 9, 1, 3, 5, 7];

        Image::from_vec(3, 3, 4, true, pixels)
    }

    #[test]
    fn construction_test() {
        let info = ImageInfo::new(3, 3, 4, true);
        let pixels =
            vec![1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                 1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6,
                 1, 2, 3, 4, 2, 3, 4, 10, 3, 4, 5, 6];
        let pixels_vec =
            vec![vec![1, 2, 3, 4], vec![2, 3, 4, 5], vec![3, 4, 5, 6],
                 vec![1, 2, 3, 4], vec![2, 3, 4, 5], vec![3, 4, 5, 6],
                 vec![1, 2, 3, 4], vec![2, 3, 4, 10], vec![3, 4, 5, 6]];
        let pixels_slice: Vec<&[i32]> =
            vec![&[1, 2, 3, 4], &[2, 3, 4, 5], &[3, 4, 5, 6],
              &[1, 2, 3, 4], &[2, 3, 4, 5], &[3, 4, 5, 6],
              &[1, 2, 3, 4], &[2, 3, 4, 10], &[3, 4, 5, 6]];
        let pixels_blank =
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,];

        let img_from_slice = Image::from_slice(3, 3, 4, true, &pixels);
        assert_eq!(img_from_slice.info(), info);
        assert_eq!(img_from_slice.data(), pixels);

        let img_from_vec = Image::from_vec(3, 3, 4, true, pixels.clone());
        assert_eq!(img_from_vec.info(), info);
        assert_eq!(img_from_vec.data(), pixels);

        let img_from_vec_of_vec = Image::from_vec_of_vec(3, 3, 4, true, pixels_vec);
        assert_eq!(img_from_vec_of_vec.info(), info);
        assert_eq!(img_from_vec_of_vec.data(), pixels);

        let img_from_vec_of_slice = Image::from_vec_of_slice(3, 3, 4, true, pixels_slice);
        assert_eq!(img_from_vec_of_slice.info(), info);
        assert_eq!(img_from_vec_of_slice.data(), pixels);

        let img_blank: Image<i32> = Image::blank(info);
        assert_eq!(img_blank.info(), info);
        assert_eq!(img_blank.data(), pixels_blank);

        let img_empty: Image<i32>= Image::empty(info);
        assert_eq!(img_empty.info(), info);
        assert!(img_empty.data().is_empty());
    }

    #[test]
    fn getters_test() {
        let img = setup();

        // Test pixel getters
        assert_eq!([3, 5, 7, 9], img.get_pixel(1, 2));
        assert_eq!([4, 3, 2, 1], img.get_pixel(2, 1));
        assert_eq!([1, 2, 3, 4], img.get_pixel(0, 0));
        assert_eq!([1, 3, 5, 7], img.get_pixel(2, 2));

        assert_eq!([3, 5, 7, 9], img.get_pixel_unchecked(1, 2));
        assert_eq!([4, 3, 2, 1], img.get_pixel_unchecked(2, 1));
        assert_eq!([1, 2, 3, 4], img.get_pixel_unchecked(0, 0));
        assert_eq!([1, 3, 5, 7], img.get_pixel_unchecked(2, 2));

        // Test index
        assert_eq!([2, 3, 4, 5], img[1]);
        assert_eq!(9, img[7][3]);
    }

    #[test]
    fn setters_test() {
        let mut img = setup();
        let pixel = [1, 1, 1, 1];

        img.set_pixel(1, 2, &pixel);
        assert_eq!(pixel, img.get_pixel(1, 2));
        img.set_pixel(2, 1, &pixel);
        assert_eq!(pixel, img.get_pixel(2, 1));
        img.set_pixel(0, 0, &pixel);
        assert_eq!(pixel, img.get_pixel(0, 0));
        img.set_pixel(2, 2, &pixel);
        assert_eq!(pixel, img.get_pixel(2, 2));

        img.set_pixel_indexed(3, &pixel);
        assert_eq!(pixel, img[3]);
    }

    #[test]
    fn get_subimage_test() {
        let img = setup();

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
    fn map_test() {
        let mut img1: Image<u8> = Image::from_slice(2, 2, 4, true,
                                                    &[1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2]);
        let img2: Image<u8> = Image::from_slice(2, 2, 4, false,
                                                &[1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2]);

        // Test map_pixels()
        let map1 = img1.map_pixels(|channels, vec| {
            for channel in channels.iter() {
                vec.push(channel + 5);
            }
        });
        assert_eq!(&[6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], map1.data());

        // Test map_pixels_if_alpha()
        let map2 = img1.map_pixels_if_alpha(|channels, vec| {
            for channel in channels.iter() {
                vec.push(channel + 5);
            }
        }, |a| a);
        let map3 = img2.map_pixels_if_alpha(|channels, vec| {
            for channel in channels.iter() {
                vec.push(channel + 5);
            }
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
    fn apply_test() {
        let mut img1: Image<u8> = Image::from_slice(2, 2, 4, true,
                                                    &[1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2]);
        let mut img2: Image<u8> = Image::from_slice(2, 2, 4, false,
                                                    &[1, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 5, 4, 3, 2]);

        // Test apply_pixels()
        img1.apply_pixels(|channels, vec| {
            for channel in channels.iter() {
                vec.push(channel + 5);
            }
        });
        assert_eq!(&[6, 7, 8, 9, 7, 8, 9, 10, 11, 10, 9, 8, 10, 9, 8, 7], img1.data());

        // Test apply_pixels_if_alpha()
        img1.apply_pixels_if_alpha(|channels, vec| {
            for channel in channels.iter() {
                vec.push(channel + 5);
            }
        }, |a| a);
        img2.apply_pixels_if_alpha(|channels, vec| {
            for channel in channels.iter() {
                vec.push(channel + 5);
            }
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
}

#[cfg(test)]
mod sub_image_tests {
    use crate::image::{BaseImage, SubImage};

    #[test]
    fn subimage_test() {
        let subimg = SubImage::new(2, 2, 3, false,
                                   vec![&[1, 2, 3], &[4, 5, 6], &[6, 5, 4], &[3, 2, 1]]);

        let pixel = [6, 5, 4];
        assert_eq!(pixel, subimg[2]);
        assert_eq!(pixel, subimg.get_pixel(0, 1));
    }
}

#[cfg(test)]
mod pixel_array_impl_tests {
    use crate::image::Pixel;

    #[test]
    fn general_test() {
        let pixel = [1, 2, 3, 4];

        // Test basic methods
        assert_eq!(4, pixel.alpha());
        assert_eq!([1, 2, 3], pixel.channels_without_alpha());
    }

    #[test]
    fn map_test() {
        let mut pixel = [1, 2, 3, 4];

        // Test map()
        assert_eq!(vec![6, 7, 8, 9], pixel.map_all(|c| c + 5));

        // Test map_alpha()
        assert_eq!(vec![6, 7, 8, 4], pixel.map_alpha(|c| c + 5, |a| a));

        // Test apply()
        pixel.apply(|c| c + 5);
        assert_eq!([6, 7, 8, 9], pixel);

        // Test apply_alpha()
        pixel.apply_alpha(|c| c - 5, |a| a);
        assert_eq!([1, 2, 3, 9], pixel);
    }

    #[test]
    fn is_black_test() {
        let pixel = [1, 2, 3, 4];
        let pixel_black = [0, 0, 0];
        let pixel_black_alpha = [0, 0, 0, 1];

        assert!(!pixel.is_black());
        assert!(pixel_black.is_black());
        assert!(!pixel_black_alpha.is_black());

        assert!(!pixel.is_black_alpha());
        assert!(pixel_black.is_black_alpha());
        assert!(pixel_black_alpha.is_black_alpha());
    }
}