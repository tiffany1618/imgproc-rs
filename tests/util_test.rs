use imgproc_rs::image::Image;
use imgproc_rs::util;

#[test]
fn summed_area_table_test() {
    let input = Image::from_slice(6, 3, 1, false,
                                  &[31.0, 2.0, 4.0, 33.0, 5.0, 36.0,
                               12.0, 26.0, 9.0, 10.0, 29.0, 25.0,
                               13.0, 17.0, 21.0, 22.0, 20.0, 18.0]);
    let output = util::summed_area_table(&input);
    let output_table = [31.0, 33.0, 37.0, 70.0, 75.0, 111.0,
                                    43.0, 71.0, 84.0, 127.0, 161.0, 222.0,
                                    56.0, 101.0, 135.0, 200.0, 254.0, 333.0];

    assert_eq!(output_table, output.data());
}

#[test]
fn rectangular_intensity_sum_test() {
    let table = [31.0, 33.0, 37.0, 70.0, 75.0, 111.0,
                           43.0, 71.0, 84.0, 127.0, 161.0, 222.0,
                           56.0, 101.0, 135.0, 200.0, 254.0, 333.0];
    let input = Image::from_slice(6, 4, 1, false, &table);

    assert_eq!(vec![154.0], util::rectangular_intensity_sum(&input, 1, 1, 4, 2));
    assert_eq!(vec![198.0], util::rectangular_intensity_sum(&input, 1, 0, 4, 2));
    assert_eq!(vec![254.0], util::rectangular_intensity_sum(&input, 0, 0, 4, 2));
}