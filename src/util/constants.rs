//! A module for useful image constants

/////////////////////////////////////////
// Colorspace transformation constants
/////////////////////////////////////////

/// Gamma value for sRGB images
pub const GAMMA: f32 = 2.2;

/// sRGB to CIE XYZ transformation matrix
pub const SRGB_TO_XYZ_MAT: [f32; 9] = [0.4124564, 0.3575761, 0.1804375,
                                       0.2126729, 0.7151522, 0.0721750,
                                       0.0193339, 0.1191920, 0.9503041];

/// CIE XYZ to sRGB transformation matrix
pub const XYZ_TO_SRGB_MAT: [f32; 9] = [3.2404542, -1.5371385, -0.4985314,
                                       -0.9692660, 1.8760108, 0.0415560,
                                       0.0556434, -0.2040259, 1.0572252];

/////////////////////////////
// Image filtering kernels
/////////////////////////////

/// 2D 3x3 kernel for Gaussian blur
pub const K_GAUSSIAN_BLUR_2D_3: [f32; 9] = [1.0/16.0, 2.0/16.0, 1.0/16.0,
                                            2.0/16.0, 4.0/16.0, 2.0/16.0,
                                            1.0/16.0, 2.0/16.0, 1.0/16.0];

/// 1D kernel of length 3 for Gaussian blur
pub const K_GAUSSIAN_BLUR_1D_3: [f32; 3] = [1.0/4.0, 2.0/4.0, 1.0/4.0];

/// 2D 5x5 kernel for Gaussian blur
pub const K_GAUSSIAN_BLUR_2D_5: [f32; 25] = [1.0/256.0, 4.0/256.0, 6.0/256.0, 4.0/256.0, 1.0/256.0,
                                             4.0/256.0, 16.0/256.0, 24.0/256.0, 16.0/256.0, 4.0/256.0,
                                             6.0/256.0, 24.0/256.0, 36.0/256.0, 24.0/256.0, 6.0/256.0,
                                             4.0/256.0, 16.0/256.0, 24.0/256.0, 16.0/256.0, 4.0/256.0,
                                             1.0/256.0, 4.0/256.0, 6.0/256.0, 4.0/256.0, 1.0/256.0];

/// 1D kernel of length 5 for Gaussian blur
pub const K_GAUSSIAN_BLUR_1D_5: [f32; 5] = [1.0/16.0, 4.0/16.0, 6.0/16.0, 4.0/16.0, 1.0/16.0];

/// 5x5 kernel for unsharp masking (based on Gaussian blur)
pub const K_UNSHARP_MASKING: [f32; 25] = [-1.0/256.0, -4.0/256.0, -6.0/256.0, -4.0/256.0, -1.0/256.0,
                                          -4.0/256.0, -16.0/256.0, -24.0/256.0, -16.0/256.0, -4.0/256.0,
                                          -6.0/256.0, -24.0/256.0, 476.0/256.0, -24.0/256.0, -6.0/256.0,
                                          -4.0/256.0, -16.0/256.0, -24.0/256.0, -16.0/256.0, -4.0/256.0,
                                          -1.0/256.0, -4.0/256.0, -6.0/256.0, -4.0/256.0, -1.0/256.0];


/// 3x3 kernel for image sharpening
pub const K_SHARPEN: [f32; 9] = [0.0, -1.0, 0.0,
                                 -1.0, 5.0, -1.0,
                                 0.0, -1.0, 0.0];

/// 1D vertical kernel for the Sobel operator
pub const K_SOBEL_1D_VERT: [f32; 3] = [1.0, 2.0, 1.0];

/// 1D horizontal kernel for the Sobel operator
pub const K_SOBEL_1D_HORZ: [f32; 3] = [-1.0, 0.0, 1.0];

/// 1D vertical kernel for the Prewitt operator
pub const K_PREWITT_1D_VERT: [f32; 3] = [1.0, 1.0, 1.0];

/// 1D horizontal kernel for the Prewitt operator
pub const K_PREWITT_1D_HORZ: [f32; 3] = [-1.0, 0.0, 1.0];

/// Laplacian discrete approximation kernel
pub const K_LAPLACIAN: [f32; 9] = [0.0, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0];