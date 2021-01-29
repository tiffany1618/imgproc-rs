# imgproc-rs

[![cargo-test](https://github.com/tiffany1618/imgproc-rs/workflows/cargo-test/badge.svg)](https://github.com/tiffany1618/imgproc-rs/actions)

A Rust image processing library.

## Supported Image Formats

* PNG
* JPEG

Hopefully, support for more image formats will be added soon.

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
