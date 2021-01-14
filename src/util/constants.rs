// Colorspace transformation constants

// Gamma value for sRGB images
pub const GAMMA: f32 = 2.2;

// sRGB to CIEXYZ transformation matrix
pub const SRGB_TO_XYZ_MAT: [f32; 9] = [0.4124564, 0.3575761, 0.1804375,
                                       0.2126729, 0.7151522, 0.0721750,
                                       0.0193339, 0.1191920, 0.9503041];

// CIEXYZ to sRGB transformation matrix
pub const XYZ_TO_SRGB_MAT: [f32; 9] = [3.2404542, -1.5371385, -0.4985314,
                                       -0.9692660, 1.8760108, 0.0415560,
                                       0.0556434, -0.2040259, 1.0572252];