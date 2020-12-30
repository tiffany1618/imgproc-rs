use rulinalg::matrix::Matrix;
use image::{ImageBuffer, GenericImage, Pixel, Rgba};
use std::collections::HashMap;

// Colorspace transformation constants
const LIN_RGB_GAMMA: f32 = 2.2;
const sRGB_TO_XYZ_MAT: Matrix<f32> = Matrix::new(3, 3, vec![0.4124564, 0.3575761, 0.1804375,
                                                            0.2126729, 0.7151522, 0.0721750,
                                                            0.0193339, 0.1191920, 0.9503041]);
const XYZ_TO_sRGB_MAT: Matrix<f32> = Matrix::new(3, 3, vec![3.2404542, -1.5371385, -0.4985314,
                                                            -0.9692660, 1.8760108, 0.0415560,
                                                            0.0556434, -0.2040259, 1.0572252]);

// // Image helper functions
// fn generate_xys_tristimulus_vals(ref_white: &str) -> Option<(f32, f32, f32)> {
//     return match ref_white {
//         "D50" | "d50" => Some((96.4212, 100.0, 82.5188)),
//         "D65" | "d65" => Some((95.0489, 100.0, 103.8840)),
//         _ => None,
//     }
// }
//
// fn xyz_to_lab_func(num: f32) -> f32 {
//     let d: f32 = 6.0 / 29.0;
//
//     if num > d.powf(3.0) {
//         num.powf(1.0 / 3.0)
//     } else {
//         (num / (3.0 * d * d)) + (4.0 / 29.0)
//     }
// }
//
// fn lab_to_xyz_func(num: f32) -> f32 {
//     let d: f32 = 6.0 / 29.0;
//
//     if num > d {
//         num.powf(3.0)
//     } else {
//         3.0 * d * d * (num - (4.0 / 29.0))
//     }
// }
//
// // Input: image in CIELAB
// fn generate_histogram_percentiles<I, P>(input: &I, percentiles: &mut HashMap<i32, f32>)
//     where I: GenericImage<Pixel=P>,
//           P: Pixel<Subpixel=f32> {
//     let precision = 255.0;
//     let mut histogram = HashMap::new();
//     let (width, height) = input.dimensions();
//
//     for y in 0..height {
//         for x in 0..width {
//             let p = (input.get_pixel(x, y).channels()[0] * precision).round() as i32;
//             let count = histogram.entry(p).or_insert(1);
//             *count += 1;
//         }
//     }
//
//     let mut sum: i32 = 0;
//     let num_pixels: u32 = width * height * 3;
//     for (key, val) in &histogram {
//         sum += val;
//         // TODO: make sure the precision multiplication works with histogram equalization function
//         percentiles.insert(*key, sum as f32 / num_pixels as f32);
//     }
// }
//
// fn create_lookup_table<T>(table: &mut [T; 256], f: fn(u8) -> T) {
//     for i in 0..256 {
//         table[i] = f(i as u8);
//     }
// }
//
// fn point_operation_lookup_table<I, T, P>(input: &I, table: &[T; 256]) -> ImageBuffer<P, Vec<T>>
//     where I: GenericImage<Pixel=Rgba<u8>>,
//           P: Pixel, {
//     let (width, height) = input.dimensions();
//     let mut output = ImageBuffer::new(width, height);
//
//     for y in 0..height {
//         for x in 0..width {
//             let p = input.get_pixel(x, y).map_without_alpha(|i|   table[i as usize]);
//             output.put_pixel(x, y, p);
//         }
//     }
//
//     output
// }
//
// fn point_operation<I, P, T, S>(input: &I, f: fn(T) -> S) -> ImageBuffer<P, Vec<S>>
//     where I: GenericImage<Pixel=P>,
//           P: Pixel {
//     let (width, height) = input.dimensions();
//     let mut output = ImageBuffer::new(width, height);
//
//     for y in 0..height {
//         for x in 0..width {
//             let p = input.get_pixel(x, y).map_without_alpha(f);
//             output.put_pixel(x, y, p);
//         }
//     }
//
//     output
// }
//
// fn linear_colorspace_change<I, P, T>(input: &I, mat: &Matrix<f32>) -> ImageBuffer<P, Vec<T>>
//     where I: GenericImage<Pixel=P>,
//           P: Pixel {
//     let (width, height) = input.dimensions();
//     let mut output = ImageBuffer::new(width, height);
//
//     for y in 0..height {
//         for x in 0..width {
//             let p_in = Matrix::new(3, 1, input.get_pixel(x, y)[0..3]);
//             let p_out = p_in * mat;
//             output.put_pixel(x, y, p_out);
//         }
//     }
//
//     output
// }
