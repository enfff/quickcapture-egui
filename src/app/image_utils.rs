// As explained in
// https://docs.rs/egui/latest/egui/struct.ColorImage.html#method.from_rgba_unmultiplied

use image::RgbaImage;
use egui::{ImageData, ColorImage};

pub fn load_image_from_memory(image_data: RgbaImage) -> ImageData {
    // let image = image::load_from_memory(image_data).expect("Error in the buffer");
    let size = [image_data.width() as _, image_data.height() as _];
    // let image_buffer = image.to_rgba8();
    let pixels: image::FlatSamples<&[u8]> = image_data.as_flat_samples();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    );
    return ImageData::from(color_image);   
}

pub fn load_image_from_memory_2(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}