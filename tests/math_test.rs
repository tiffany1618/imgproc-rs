use imgproc_rs::util::math;
use imgproc_rs::image::Pixel;
use imgproc_rs::util::constant::K_GAUSSIAN_BLUR_2D_3;

#[test]
fn vector_mul_test() {
    let mat = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec = vec![1, 2, 3];
    let res = math::vector_mul(&mat, &vec).unwrap();

    assert_eq!(vec![14, 32, 50], res);
}

#[test]
fn max_test() {
    assert_eq!(3.0, math::max(1.0, 2.0, 3.0));
    assert_eq!(1.0, math::max(1.0, 1.0, 1.0));
    assert_eq!(3.0, math::max(1.0, 3.0, 3.0));
    assert_eq!(3.0, math::max(1.0, 1.0, 3.0));
}

#[test]
fn min_test() {
    assert_eq!(1.0, math::min(1.0, 2.0, 3.0));
    assert_eq!(1.0, math::min(1.0, 1.0, 1.0));
    assert_eq!(1.0, math::min(1.0, 3.0, 3.0));
    assert_eq!(1.0, math::min(1.0, 1.0, 3.0));
}

#[test]
fn apply_1d_kernel_test() {
    let pixels = [&Pixel::new(&[1.0, 2.0, 3.0]),
                                  &Pixel::new(&[4.0, 5.0, 6.0]),
                                  &Pixel::new(&[2.0, 3.0, 4.0])];
    let kernel = [1.0, 2.0, 1.0];
    let res = math::apply_1d_kernel(&pixels, &kernel).unwrap();

    assert_eq!(Pixel::new(&[11.0, 15.0, 19.0]), res);
}

#[test]
fn apply_2d_kernel_test() {
    let pixels = [&Pixel::new(&[1.0, 2.0, 3.0]),
                      &Pixel::new(&[2.0, 3.0, 4.0]),
                      &Pixel::new(&[3.0, 4.0, 5.0]),
                      &Pixel::new(&[6.0, 5.0, 4.0]),
                      &Pixel::new(&[5.0, 4.0, 3.0]),
                      &Pixel::new(&[4.0, 3.0, 2.0]),
                      &Pixel::new(&[2.0, 4.0, 6.0]),
                      &Pixel::new(&[3.0, 5.0, 7.0]),
                      &Pixel::new(&[1.0, 3.0, 5.0])];
    let res = math::apply_2d_kernel(&pixels, &K_GAUSSIAN_BLUR_2D_3).unwrap();

    assert_eq!(Pixel::new(&[3.5625, 3.8125, 4.0625]), res);
}