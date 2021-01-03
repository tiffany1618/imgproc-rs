use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};
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
    let ref mut file_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(file_writer, width, height);
    let color_type = png_channels_to_color_type(channels)?;
    encoder.set(color_type).set(png::BitDepth::Eight);

    let mut png_writer = encoder.write_header()?;
    png_writer.write_image_data(&input.pixels_as_vector())?;

    Ok(())
}

pub fn jpg_pixel_format_to_channels(pixel_format: jpeg_decoder::PixelFormat) -> u8 {
    match pixel_format {
        jpeg_decoder::PixelFormat::L8 => 1,
        jpeg_decoder::PixelFormat::RGB24 => 3,
        jpeg_decoder::PixelFormat::CMYK32 => 4,
    }
}

fn decode_jpg(filename: &str) -> Result<Image<u8>, ImageError> {
    let file = File::open(filename)?;
    let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file));
    let pixels = decoder.decode()?;
    let info = decoder.info().ok_or(ImageError::Other("unable to read metadata".to_string()))?;
    let channels = jpg_pixel_format_to_channels(info.pixel_format);

    Ok(Image::new(info.width as u32, info.height as u32, channels, &pixels))
}

// TODO: Add support for jpg encoding
// fn encode_jpg(input: &Image<u8>, filename: &str) -> Result<(), ImageError> {
//
// }

// TODO: Add support for more image file formats
pub fn read(filename: &str) -> Result<Image<u8>, ImageError> {
    let path = Path::new(filename);
    let ext = path.extension().ok_or(ImageError::Other("could not extract file extension".to_string()))?;
    let ext_str = ext.to_str().ok_or(ImageError::Other("invalid file extension".to_string()))?;

    match ext_str.to_ascii_lowercase().as_str() {
        "png" => Ok(decode_png(filename)?),
        "jpg" | "jpeg" => Ok(decode_jpg(filename)?),
        _ => Err(ImageError::Other("unsupported file format".to_string())),
    }
}

pub fn write(input: &Image<u8>, filename: &str) -> Result<(), ImageError> {
    let path = Path::new(filename);
    let ext = path.extension().ok_or(ImageError::Other("could not extract file extension".to_string()))?;
    let ext_str = ext.to_str().ok_or(ImageError::Other("invalid file extension".to_string()))?;

    match ext_str.to_ascii_lowercase().as_str() {
        "png" => Ok(encode_png(input, path)?),
        // "jpg" | "jpeg" => Ok(encode_jpg(input, filename)?),
        _ => Err(ImageError::Other("unsupported file format".to_string())),
    }
}