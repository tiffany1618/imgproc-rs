use std::path::Path;
use std::fs::File;
use std::io;
use std::result::Result;

use png::HasParameters;
use jpeg_decoder;

use crate::image::Image;
use crate::errors::ImageError;

fn png_color_type_to_channels(color_type: png::ColorType) -> u8 {
    match color_type {
        png::ColorType::Grayscale => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::RGB => 3,
        png::ColorType::RGBA => 4,
        png::ColorType::Indexed => 0, // TODO: Fix this
    }
}

fn png_channels_to_color_type(channels: u8) -> Result<png::ColorType, ImageError> {
    match channels {
        1 => Ok(png::ColorType::Grayscale),
        2 => Ok(png::ColorType::GrayscaleAlpha),
        3 => Ok(png::ColorType::RGB),
        4 => Ok(png::ColorType::RGBA),
        _ => Err(ImageError::Other("invalid number of channels".to_string())), // TODO: Add png::ColorType::Indexed
    }
}

fn decode_png(filename: &str) -> Result<Image<u8>, ImageError> {
    let decoder = png::Decoder::new(File::open(filename)?);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    let channels = png_color_type_to_channels(info.color_type);

    Ok(Image::new(info.width, info.height, channels, &buf))
}

fn encode_png(input: &Image<u8>, path: &Path) -> Result<(), ImageError> {
    let (width, height, channels) = input.dimensions_with_channels();
    let file = File::create(path)?;
    let ref mut file_writer = io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(file_writer, width, height);
    let color_type = png_channels_to_color_type(channels)?;
    encoder.set(color_type).set(png::BitDepth::Eight);

    println!("{}", channels);

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
    //     _ => Err(ImageError::FormatError),
    // }
}

pub fn write(input: &Image<u8>, filename: &str) -> Result<(), ImageError> {
    let path = Path::new(filename);
    let ext = path.extension();

    Ok(encode_png(input, &path)?)
}