# imgproc-rs

[![crates.io](https://img.shields.io/crates/v/imgproc-rs.svg)](https://crates.io/crates/imgproc-rs)
[![Documentation](https://docs.rs/imgproc-rs/badge.svg)](https://docs.rs/imgproc-rs)
[![cargo-test](https://github.com/tiffany1618/imgproc-rs/workflows/cargo-test/badge.svg)](https://github.com/tiffany1618/imgproc-rs/actions)

A Rust image processing library.

## Supported Image Formats

`imgproc-rs` uses the i/o functions provided in the [`image`](https://github.com/image-rs/image) crate. A list of 
supported image formats can be found [here](https://docs.rs/image/0.23.12/image/codecs/index.html#supported-formats). 

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
