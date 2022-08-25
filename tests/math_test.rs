use imgproc_rs::util;
use imgproc_rs::util::constants::K_GAUSSIAN_BLUR_2D_3;
use imgproc_rs::image::SubImage;

#[test]
fn vector_mul_test() {
    let mat = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec = vec![1, 2, 3];
    let res1 = util::vector_mul(&mat, &vec).unwrap();

    let mut res2 = Vec::new();
    util::vector_mul_mut(&mat, &vec, &mut res2).unwrap();

    assert_eq!(vec![14, 32, 50], res1);
    assert_eq!(vec![14, 32, 50], res2);
}

#[test]
fn max_test() {
    // Test max_3()
    assert_eq!(3.0, util::max_3(1.0, 2.0, 3.0));
    assert_eq!(1.0, util::max_3(1.0, 1.0, 1.0));
    assert_eq!(3.0, util::max_3(1.0, 3.0, 3.0));
    assert_eq!(3.0, util::max_3(1.0, 1.0, 3.0));

    // Test max_4()
    assert_eq!(4.0, util::max_4(1.0, 2.0, 3.0, 4.0));
    assert_eq!(1.0, util::max_4(1.0, 1.0, 1.0, 1.0));
    assert_eq!(3.0, util::max_4(1.0, 2.0, 2.0, 3.0));
    assert_eq!(3.0, util::max_4(1.0, 2.0, 3.0, 3.0));
}

#[test]
fn min_test() {
    // Test min_3()
    assert_eq!(1.0, util::min_3(1.0, 2.0, 3.0));
    assert_eq!(1.0, util::min_3(1.0, 1.0, 1.0));
    assert_eq!(1.0, util::min_3(1.0, 3.0, 3.0));
    assert_eq!(1.0, util::min_3(1.0, 1.0, 3.0));

    // Test min_4()
    assert_eq!(1.0, util::min_4(1.0, 2.0, 3.0, 4.0));
    assert_eq!(1.0, util::min_4(1.0, 1.0, 1.0, 1.0));
    assert_eq!(1.0, util::min_4(1.0, 2.0, 2.0, 3.0));
    assert_eq!(1.0, util::min_4(1.0, 2.0, 3.0, 3.0));
}

#[test]
#[cfg(not(feature = "rayon"))]
fn apply_1d_kernel_test() {
    let pixels: Vec<&[f32]> = vec![&[1.0, 2.0, 3.0],
                               &[4.0, 5.0, 6.0],
                               &[2.0, 3.0, 4.0]];
    let subimg = SubImage::new(3, 1, 3, false, pixels);
    let kernel = [1.0, 2.0, 1.0];

    let mut res = Vec::new();
    util::apply_1d_kernel(&subimg, &mut res, &kernel).unwrap();

    assert_eq!(vec![11.0, 15.0, 19.0], res);
}

#[test]
#[cfg(feature = "rayon")]
fn apply_1d_kernel_test() {
    let pixels: Vec<&[f32]> = vec![&[1.0, 2.0, 3.0],
                                   &[4.0, 5.0, 6.0],
                                   &[2.0, 3.0, 4.0]];
    let subimg = SubImage::new(3, 1, 3, false, pixels);
    let kernel = [1.0, 2.0, 1.0];

    let res = util::apply_1d_kernel(&subimg, &kernel).unwrap();

    assert_eq!(vec![11.0, 15.0, 19.0], res);
}

#[test]
#[cfg(not(feature = "rayon"))]
fn apply_2d_kernel_test() {
    let pixels: Vec<&[f32]> = vec![&[1.0, 2.0, 3.0],
                      &[2.0, 3.0, 4.0],
                      &[3.0, 4.0, 5.0],
                      &[6.0, 5.0, 4.0],
                      &[5.0, 4.0, 3.0],
                      &[4.0, 3.0, 2.0],
                      &[2.0, 4.0, 6.0],
                      &[3.0, 5.0, 7.0],
                      &[1.0, 3.0, 5.0]];
    let subimg = SubImage::new(3, 3, 3, false, pixels);

    let mut res = Vec::new();
    util::apply_2d_kernel(&subimg, &mut res, &K_GAUSSIAN_BLUR_2D_3).unwrap();

    assert_eq!(vec![3.5625, 3.8125, 4.0625], res);
}

#[test]
#[cfg(feature = "rayon")]
fn apply_2d_kernel_test() {
    let pixels: Vec<&[f32]> = vec![&[1.0, 2.0, 3.0],
                                   &[2.0, 3.0, 4.0],
                                   &[3.0, 4.0, 5.0],
                                   &[6.0, 5.0, 4.0],
                                   &[5.0, 4.0, 3.0],
                                   &[4.0, 3.0, 2.0],
                                   &[2.0, 4.0, 6.0],
                                   &[3.0, 5.0, 7.0],
                                   &[1.0, 3.0, 5.0]];
    let subimg = SubImage::new(3, 3, 3, false, pixels);

    let res = util::apply_2d_kernel(&subimg, &K_GAUSSIAN_BLUR_2D_3).unwrap();

    assert_eq!(vec![3.5625, 3.8125, 4.0625], res);
}
