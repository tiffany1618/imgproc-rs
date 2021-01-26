use imgproc_rs::image::Pixel;

#[test]
fn pixel_general_test() {
    let pixel = [1, 2, 3, 4];

    // Test basic methods
    assert_eq!(4, pixel.alpha());
    assert_eq!([1, 2, 3], pixel.channels_without_alpha());
}

#[test]
fn pixel_map_test() {
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