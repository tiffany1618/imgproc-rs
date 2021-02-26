use imgproc_rs::error::ImgIoResult;
use imgproc_rs::io::read;
use imgproc_rs::image::{BaseImage, Image};

const PATH: &str = "images/yosemite.jpg";

pub fn setup() -> ImgIoResult<Image<u8>> {
    let img = read(PATH)?;

    println!("Reading: {}", PATH);
    println!("{}", img.info());

    Ok(img)
}