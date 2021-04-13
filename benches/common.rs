use imgproc_rs::error::ImgIoResult;
use imgproc_rs::io::read;
use imgproc_rs::image::{BaseImage, Image};

pub fn setup(path: &str) -> ImgIoResult<Image<u8>> {
    let img = read(path)?;

    println!("Reading: {}", path);
    println!("{}", img.info());

    Ok(img)
}