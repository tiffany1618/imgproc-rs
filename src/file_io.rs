use std::path::Path;
use std::fs::File;
use std::io;
use std::result::Result;

use png;
use jpeg_decoder;

use crate::image::Image;
use crate::errors::ImageError;

fn png_color_type_to_channels(color_type: png::ColorType) -> u8 {
    match color_type {
        png::ColorType::Grayscale => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::RGB => 3,
        png::ColorType::RGBA => 4,
        png::ColorType::Indexed => 0, // TODO: fix this
    }
}

fn decode_png(filename: &str) -> Result<Image<u8>, ImageError> {
    let decoder = png::Decoder::new(File::open(filename)?);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    let channels = png_color_type_to_channels(info.color_type) as u32;

    Ok(Image::new(info.width, info.height, channels, &buf))
}

fn encode_png(input: &Image<u8>, path: &Path) -> Result<(), ImageError> {
    let (width, height) = input.dimensions();
    let file = File::create(path)?;
    let ref mut file_writer = io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(file_writer, width, height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);

    let mut png_writer = encoder.write_header()?;
    png_writer.write_image_data(&input.pixels_as_vector())?;

    Ok(())
}

// fn decode_jpg(filename: &str) -> Result<Image<u8>, ImageError> {
//
// }
//
// fn encode_jpg(input: &Image<u8>, filename: &str) -> Result<(), ImageError> {
//
// }

pub fn read(filename: &str) -> Result<Image<u8>, ImageError> {
    let path = Path::new(filename);
    let ext = path.extension();

    Ok(decode_png(filename)?)

    // match ext {
    //     "png" => decode_png(filename)?,
    //     "jpg" => decode_jpg(filename)?,
    //     _ => Err(DecodingError::FormatError),
    // }
}

pub fn write(input: &Image<u8>, filename: &str) -> Result<(), ImageError> {
    let path = Path::new(filename);
    let ext = path.extension();

    Ok(encode_png(input, &path)?)
}