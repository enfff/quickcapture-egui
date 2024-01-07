use image::RgbaImage;
use egui::ImageData;

// As explained in
// https://docs.rs/egui/latest/egui/struct.ColorImage.html#method.from_rgba_unmultiplied

pub fn load_image_from_memory(image_data: RgbaImage) -> ImageData {
    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [image_data.width() as _, image_data.height() as _],
        image_data.as_flat_samples().as_slice(),
    );
    return ImageData::from(color_image);   
}

