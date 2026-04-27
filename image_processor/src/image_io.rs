use image::{DynamicImage, ImageBuffer, ImageReader, Rgba};
use log::info;
use std::path::Path;

pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn load_image<P: AsRef<Path>>(path: P) -> Result<ImageData, image::ImageError> {
    let img = ImageReader::open(path.as_ref())?.decode()?;
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();
    let data = img.into_raw();
    let img_data = ImageData {
        width,
        height,
        data,
    };

    info!(
        "Loaded image <{}>: width {}, height {}",
        path.as_ref().display(),
        img_data.width,
        img_data.height
    );
    Ok(img_data)
}

pub fn save_image<P: AsRef<Path>>(path: P, img_data: ImageData) -> Result<(), image::ImageError> {
    let buffer =
        ImageBuffer::<Rgba<u8>, _>::from_raw(img_data.width, img_data.height, img_data.data)
            .ok_or_else(|| {
                image::ImageError::Parameter(image::error::ParameterError::from_kind(
                    image::error::ParameterErrorKind::Generic(String::from(
                        "Не удалось создать ImageBuffer из сырых данных",
                    )),
                ))
            })?;

    let dynamic = DynamicImage::ImageRgba8(buffer);
    dynamic.save(path.as_ref())?;

    info!("Result image saved at: <{}>", path.as_ref().display());
    Ok(())
}
