use imgproc_rs::util::math;
use imgproc_rs::util::constants::K_GAUSSIAN_BLUR_2D_3;
use imgproc_rs::image::SubImage;
use core::num::FpCategory::Subnormal;

#[test]
fn vector_mul_test() {
    let mat = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec = vec![1, 2, 3];
    let res = math::vector_mul(&mat, &vec).unwrap();

    assert_eq!(vec![14, 32, 50], res);
}

#[test]
fn max_test() {
    // Test max_3()
    assert_eq!(3.0, math::max_3(1.0, 2.0, 3.0));
    assert_eq!(1.0, math::max_3(1.0, 1.0, 1.0));
    assert_eq!(3.0, math::max_3(1.0, 3.0, 3.0));
    assert_eq!(3.0, math::max_3(1.0, 1.0, 3.0));

    // Test max_4()
    assert_eq!(4.0, math::max_4(1.0, 2.0, 3.0, 4.0));
    assert_eq!(1.0, math::max_4(1.0, 1.0, 1.0, 1.0));
    assert_eq!(3.0, math::max_4(1.0, 2.0, 2.0, 3.0));
    assert_eq!(3.0, math::max_4(1.0, 2.0, 3.0, 3.0));
}

#[test]
fn min_test() {
    // Test min_3()
    assert_eq!(1.0, math::min_3(1.0, 2.0, 3.0));
    assert_eq!(1.0, math::min_3(1.0, 1.0, 1.0));
    assert_eq!(1.0, math::min_3(1.0, 3.0, 3.0));
    assert_eq!(1.0, math::min_3(1.0, 1.0, 3.0));

    // Test min_4()
    assert_eq!(1.0, math::min_4(1.0, 2.0, 3.0, 4.0));
    assert_eq!(1.0, math::min_4(1.0, 1.0, 1.0, 1.0));
    assert_eq!(1.0, math::min_4(1.0, 2.0, 2.0, 3.0));
    assert_eq!(1.0, math::min_4(1.0, 2.0, 3.0, 3.0));
}

#[test]
fn apply_1d_kernel_test() {
    let pixels: Vec<&[f64]> = vec![&[1.0, 2.0, 3.0],
                               &[4.0, 5.0, 6.0],
                               &[2.0, 3.0, 4.0]];
    let subimg = SubImage::new(3, 1, 3, false, pixels);
    let kernel = [1.0, 2.0, 1.0];
    let res = math::apply_1d_kernel(subimg, &kernel).unwrap();

    assert_eq!(vec![11.0, 15.0, 19.0], res);
}

#[test]
fn apply_2d_kernel_test() {
    let pixels: Vec<&[f64]> = vec![&[1.0, 2.0, 3.0],
                      &[2.0, 3.0, 4.0],
                      &[3.0, 4.0, 5.0],
                      &[6.0, 5.0, 4.0],
                      &[5.0, 4.0, 3.0],
                      &[4.0, 3.0, 2.0],
                      &[2.0, 4.0, 6.0],
                      &[3.0, 5.0, 7.0],
                      &[1.0, 3.0, 5.0]];
    let subimg = SubImage::new(3, 3, 3, false, pixels);
    let res = math::apply_2d_kernel(subimg, &K_GAUSSIAN_BLUR_2D_3).unwrap();

    assert_eq!(vec![3.5625, 3.8125, 4.0625], res);
}