use crate::util::Number;
use crate::image::Pixel;

/// Returns the result of the multiplication of a square matrix by a vector
pub fn vector_mul<T: Number>(mat: &[T], vec: &[T]) -> Option<Vec<T>> {
    let rows = vec.len();
    let mat_cols = mat.len() / rows;

    // Check for valid dimensions
    if mat_cols != rows {
        return None;
    }

    let mut output = vec![0.into(); rows];

    for i in 0..rows {
        for j in 0..rows {
            output[i] += mat[rows * i + j] * vec[j];
        }
    }

    Some(output)
}

/// Applies a 1D kernel to `pixels`
///
/// # Arguments
///
/// * `pixels` - a slice of `Pixel` references
/// * `kernel` - a slice representing the 1D kernel to be applied; must be of odd dimensions
pub fn apply_1d_kernel(pixels: &[&Pixel<f64>], kernel: &[f64]) -> Option<Pixel<f64>> {
    let size = pixels.len();
    let num_channels = pixels[0].num_channels() as usize;

    // Check for valid dimensions
    if size % 2 == 0 || kernel.len() != size {
        return None;
    }

    let mut vec = vec![0.0; num_channels];

    // Apply kernel
    for i in 0..size {
        for j in 0..num_channels {
            vec[j] += kernel[i] * pixels[i].channels()[j];
        }
    }

    Some(Pixel::new(&vec))
}

/// Applies a 2D kernel to `pixels`
///
/// # Arguments
///
/// * `pixels` - a slice of `Pixel` references
/// * `kernel` - a slice representing the 2D kernel to be applied; must have odd dimensions
pub fn apply_2d_kernel(pixels: &[&Pixel<f64>], kernel: &[f64]) -> Option<Pixel<f64>> {
    let size = (pixels.len() as f32).sqrt() as usize;
    let num_channels = pixels[0].num_channels() as usize;

    // Check for valid dimensions
    if size % 2 == 0 || kernel.len() != size * size {
        return None;
    }

    let mut vec = vec![0.0; num_channels];

    // Apply kernel
    for y in 0..size {
        for x in 0..size {
            let index = y * size + x;
            for j in 0..num_channels {
                vec[j] += kernel[index] * pixels[index].channels()[j];
            }
        }
    }

    Some(Pixel::new(&vec))
}