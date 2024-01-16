use super::screenshot_view::ScreenshotView;
use crate::app::ScreenshotType;
use image::{GenericImage, RgbaImage, GenericImageView};
use screenshots::Screen;
use std::io::Cursor;

struct ScreenImage {
    screen: Screen,
    image: screenshots::Image,
}

// Fa lo screenshot di ogni schermo e ritorna un buffer che descrive l'immagine. Ma non è l'immagine stessa.
pub fn take_screenshot(
    _screenshot_type: Option<ScreenshotType>,
    _grabbed_area: Option<ScreenshotView>,
    _ctx: &egui::Context,
) -> Option<image::RgbaImage> {
    // TODO
    // Virtual desktops don't count as screens
    let mut img: RgbaImage;
    let screen_images = Screen::all()
        .unwrap()
        .into_iter()
        .map(|screen| {
            let image = screen.capture().unwrap();
            ScreenImage { screen, image }
        })
        .collect::<Vec<ScreenImage>>();
    let x_min = screen_images
        .iter()
        .map(|s| s.screen.display_info.x * s.screen.display_info.scale_factor as i32)
        .min()
        .unwrap();
    let y_min = screen_images
        .iter()
        .map(|s| s.screen.display_info.y * s.screen.display_info.scale_factor as i32)
        .min()
        .unwrap();
    let x_max = screen_images
        .iter()
        .map(|s| (s.screen.display_info.x + s.screen.display_info.width as i32) * s.screen.display_info.scale_factor as i32)
        .max()
        .unwrap();
    let y_max = screen_images
        .iter()
        .map(|s| (s.screen.display_info.y + s.screen.display_info.height as i32) * s.screen.display_info.scale_factor as i32)
        .max()
        .unwrap();

    let offset = (x_min, y_min);
    println!("Offset: {:?}", offset);
    let size: (u32, u32);
    size = ((x_max - x_min) as u32, (y_max - y_min) as u32);
    println!("Size: {:?}", size);
    img = RgbaImage::new(size.0, size.1);
    for pixels in img.enumerate_pixels_mut() {
        *pixels.2 = image::Rgba([0, 0, 0, 255]);
    }
    for screen_image in screen_images {
        println!("Screen: {:?}", screen_image.screen.display_info);
            let screenshot = image::io::Reader::new(Cursor::new(screen_image.image.to_png().unwrap()))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            /*if screen_image.screen.display_info.scale_factor != 1.0 {
                println!(
                    "Scale factor: {}",
                    screen_image.screen.display_info.scale_factor
                );
                let scaled_screenshot = screenshot.resize(
                    (screenshot.width() as f32 / screen_image.screen.display_info.scale_factor)
                        as u32,
                    (screenshot.height() as f32 / screen_image.screen.display_info.scale_factor)
                        as u32,
                    image::imageops::FilterType::Nearest,
                );
                screenshot = scaled_screenshot;
            }*/

            let x = (screen_image.screen.display_info.x * screen_image.screen.display_info.scale_factor as i32 - offset.0) as u32;
            let y = (screen_image.screen.display_info.y * screen_image.screen.display_info.scale_factor as i32 - offset.1) as u32;
            if x + screenshot.width() <= img.width() && y + screenshot.height() <= img.height() {
                    match img.copy_from(&screenshot, x, y) {
                        Ok(_) => (),
                        Err(e) => println!("Failed to copy screen image: {}", e),
                    }
            }   
    }
    if _screenshot_type.clone().unwrap() == ScreenshotType::PartialScreen{
        let grab = _grabbed_area.clone().unwrap();
        let x_start: i32;
        let y_start: i32;
        println!("Pixels per point: {}", _ctx.pixels_per_point());
        if grab.starting_point.x > grab.ending_point.x {
            x_start = (grab.ending_point.x * _ctx.pixels_per_point()) as i32 as i32 - offset.0 as i32;
        } else {
            x_start = (grab.starting_point.x * _ctx.pixels_per_point()) as i32 as i32 - offset.0 as i32;
        }
        if grab.starting_point.y < grab.ending_point.y {
            y_start = (grab.starting_point.y * _ctx.pixels_per_point()) as i32 as i32 - offset.1 as i32;
        } else {
            y_start = (grab.ending_point.y * _ctx.pixels_per_point()) as i32 as i32 - offset.1 as i32;
        }
        let clone = img.clone();
        println!("grab dimensions: {:?}", grab.dimension_selected);
        let (real_x, real_y) = (grab.dimension_selected.x * _ctx.pixels_per_point(), grab.dimension_selected.y * _ctx.pixels_per_point());
        let cropped = image::imageops::crop_imm(&clone, x_start as u32, y_start as u32, real_x as u32, real_y as u32);
        println!("cropped dimensions: {:?}", cropped.dimensions());
        let cropped_to_img = cropped.to_image();
        img = RgbaImage::new(real_x as u32, real_y as u32);
        match img.copy_from(&cropped_to_img, 0, 0) {
            Ok(_) => (),
            Err(e) => println!("Failed to copy screen image: {}", e),
        }
    }
    let return_img: Option<RgbaImage> = Some(img);
    if return_img.is_some() {
        println!("Screenshot taken");
        return return_img;
    } else {
        println!("Screenshot failed");
        return None;
    }
}
