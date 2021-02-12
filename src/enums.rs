//! A module for image enums

/// An enum for image tone operations
pub enum Tone {
    /// Tone operation should be carried out using an RGB channels
    Rgb,

    /// Tone operation should be carried out using XYZ channels
    Xyz,
}

/// An enum for reference white values
pub enum White {
    D50,
    D65,
}

/// An enum for image thresholding operations
pub enum Thresh {
    /// If pixel value is greater than `threshold`, it is set to `max`; otherwise, it is set to 0
    Binary,

    /// If pixel value is greater than `threshold`, it is set to 0; otherwise, it is set to `max`
    BinaryInv,

    /// If pixel value is greater than `threshold`, it is set to `threshold`; otherwise, it is unchanged
    Trunc,

    /// If pixel value is greater than `threshold`, it is unchanged; otherwise, it is set to 0
    ToZero,

    /// If pixel value is greater than `threshold`, it is set to 0; otherwise, it is unchanged
    ToZeroInv,
}

/// An enum for different scaling algorithms
pub enum Scale {
    /// Nearest neighbor interpolation
    NearestNeighbor,

    /// Bilinear interpolation
    Bilinear,

    /// Bicubic interpolation
    Bicubic,

    /// Lanczos resampling with kernel size of 3
    Lanczos,
}

/// An enum for image reflection axes
pub enum Refl {
    /// Reflection axis along the line x = 0
    Vertical,

    /// Reflection axis along the line y = 0
    Horizontal,
}

/// An enum for bilateral filter algorithms
// Read a description of the algorithms here:
// https://people.csail.mit.edu/sparis/publi/2009/fntcgv/Paris_09_Bilateral_filtering.pdf
pub enum Bilateral {
    /// Direct bilateral filter algorithm
    Direct,

    /// Bilateral grid algorithm
    Grid,

    /// Local histogram algorithm
    LocalHistogram,
}