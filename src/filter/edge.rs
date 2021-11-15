////////////////////
// Edge detection
////////////////////

use crate::{filter, error, util, convert};
use crate::image::{Image, BaseImage};
use crate::error::ImgProcResult;
use crate::util::constants::{K_PREWITT_1D_VERT, K_PREWITT_1D_HORZ, K_SOBEL_1D_VERT, K_SOBEL_1D_HORZ, K_LAPLACIAN};

/// Applies a separable derivative mask to a grayscale image
pub fn derivative_mask(input: &Image<f32>, vert_kernel: &[f32], horz_kernel: &[f32]) -> ImgProcResult<Image<f32>> {
    error::check_grayscale(input)?;

    let img_x = filter::separable_filter(&input, &vert_kernel, &horz_kernel)?;
    let img_y = filter::separable_filter(&input, &horz_kernel, &vert_kernel)?;

    let mut output = Image::blank(input.info());

    for i in 0..(output.info().full_size() as usize) {
        output.set_pixel_indexed(i, &[(img_x[i][0].powf(2.0) + img_y[i][0].powf(2.0)).sqrt()]);
    }

    Ok(output)
}

/// Applies the Prewitt operator to a grayscale image
pub fn prewitt(input: &Image<f32>) -> ImgProcResult<Image<f32>> {
    Ok(derivative_mask(input, &K_PREWITT_1D_VERT, &K_PREWITT_1D_HORZ)?)
}

/// Applies the Sobel operator to a grayscale image
pub fn sobel(input: &Image<f32>) -> ImgProcResult<Image<f32>> {
    Ok(derivative_mask(input, &K_SOBEL_1D_VERT, &K_SOBEL_1D_HORZ)?)
}

/// Applies a Sobel operator with weight `weight` to a grayscale image
pub fn sobel_weighted(input: &Image<f32>, weight: u32) -> ImgProcResult<Image<f32>> {
    let vert_kernel = vec![1.0, weight as f32, 1.0];
    Ok(derivative_mask(input, &vert_kernel, &K_SOBEL_1D_HORZ)?)
}

/// Applies the Laplacian operator to a grayscale image. Output contains positive
/// and negative values - use [`normalize_laplacian()`](fn.normalize_laplacian.html) for visualization
pub fn laplacian(input: &Image<f32>) -> ImgProcResult<Image<f32>> {
    Ok(filter::unseparable_filter(input, &K_LAPLACIAN)?)
}

/// Applies the Laplacian of Gaussian operator using a `size x size` kernel to a grayscale image.
/// Output contains positive and negative values - use
/// [`normalize_laplacian()`](fn.normalize_laplacian.html) for visualization
pub fn laplacian_of_gaussian(input: &Image<f32>, size: u32, sigma: f32) -> ImgProcResult<Image<f32>> {
    let kernel = util::generate_log_kernel(size, sigma)?;
    Ok(filter::unseparable_filter(input, &kernel)?)
}

/// Normalizes the result of a Laplacian or Laplacian of Gaussian operator to the range [0, 255]
pub fn normalize_laplacian(input: &Image<f32>) -> ImgProcResult<Image<u8>> {
    error::check_grayscale(input)?;

    let min = *input.data().iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let max = *input.data().iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

    Ok(convert::scale_channels(&input, min, 0.0, max, 255.0)?.into())
}