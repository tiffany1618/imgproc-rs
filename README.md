# imgproc-rs

[![crates.io](https://img.shields.io/crates/v/imgproc-rs.svg)](https://crates.io/crates/imgproc-rs)
[![Documentation](https://docs.rs/imgproc-rs/badge.svg)](https://docs.rs/imgproc-rs)
[![cargo-test](https://github.com/tiffany1618/imgproc-rs/workflows/cargo-test/badge.svg)](https://github.com/tiffany1618/imgproc-rs/actions)

A Rust image processing library.

## Features
* Multithreading support for some functions via [rayon](https://github.com/rayon-rs/rayon) (see 
  [Enabling Multithreading](#enabling-multithreading) for more information)

## Supported Image Formats

`imgproc-rs` uses the i/o functions provided in the [`image`](https://github.com/image-rs/image) crate. A list of 
supported image formats can be found [here](https://docs.rs/image/0.23.12/image/codecs/index.html#supported-formats). 

## Notes
Running with the `release` profile greatly increases performance:
```
cargo run --release
```

## Examples

### Reading and Writing Images
```rust
use imgproc_rs::io;

fn main() {
    // Read an image from a path
    let img = io::read("path/to/some_image.png").unwrap();
    
    // Print image information
    println!("{:?}", img.info());
    
    // Write the image to a path as a PNG
    io::write(&img, "path/to/save_image.png").unwrap();
}
```

### Creating Images

Images can be created from existing vectors, slices, vectors of vectors, and vectors of slices. A few examples are shown
below.

```rust
use imgproc_rs::image::{Image, ImageInfo};

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6,
                   7, 8, 9, 10, 11, 12];

    // Create an image from a slice
    let img_slice = Image::from_slice(2, 2, 3, false, &vec);

    // Create an image from a vector
    let img_vec = Image::from_vec(2, 2, 3, false, vec);

    // Create a blank (black) image
    let img_blank: Image<u8> = Image::blank(ImageInfo::new(2, 2, 3, false));

    // Create an empty image
    let img_empty: Image<u8> = Image::empty(ImageInfo::new(2, 2, 3, false));
}
```

### Getting Image Information
```rust
use imgproc_rs::io;
use imgproc_rs::image::{Image, ImageInfo};

fn main() {
    let img = io::read("path/to/some_image.png").unwrap();

    // Get width and height of image
    let (width, height) = img.info().wh();

    // Get width, height, and channels of image
    let (width, height, channels) = img.info().whc();

    // Get width, height, channels, and alpha of image
    let (width, height, channels, alpha) = img.info().whca();

    /* Print image information
     * Example output:
     *
     * width: 2
     * height: 2
     * channels: 3
     * alpha: false
     *
     */
    println!("{}", img.info());
}
```

### Getting/Setting Image Pixels

Image pixels can be accessed using either 1D or 2D indexing. 1D indexing reads the image data row by row from left to 
right, starting in the upper left corner of the image. 2D coordinates start at zero in the upper left corner of the
image and increase downwards and to the right.

```rust
use imgproc_rs::io;
use imgproc_rs::image::{Image, BaseImage};

fn main() {
    let img = io::read("path/to/some_image.png").unwrap();
    
    // Set an image pixel using a 1D index
    img.set_pixel_indexed(0, &[1, 1, 1]);
    
    // Get an image pixel using a 1D index
    let pixel_1d = &img[0];
    
    // Set an image pixel using 2D coordinates
    img.set_pixel(1, 1, &[1, 1, 1]);
    
    // Get an image pixel using 2D coordinates
    let pixel_2d = img.get_pixel(1, 1);
}
```

## Enabling Multithreading

To enable multithreading, include the `parallel` feature in your `Cargo.toml`:

```toml
[dependencies.imgproc-rs]
version = "0.2.3"
default-features = false
features = ["parallel"]
```

Alternatively, pass the features flag to `cargo run`:

```
cargo run --features parallel
```

### Image processing functions that support multithreading:
* `transform` module
  * `crop`
  * `scale`
  * `scale_lanczos`
* All functions in the `filter` module, except:
  * `threshold`
  * `residual`
  * `median_filter`
  * `alpha_trimmed_mean_filter`