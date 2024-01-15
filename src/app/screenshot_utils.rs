use image::{GenericImage, RgbaImage};
use std::io::Cursor;
use crate::app::ScreenshotType;
use screenshots::Screen;

struct ScreenImage{
    screen: Screen,
    image: screenshots::Image,
}

// Fa lo screenshot di ogni schermo e ritorna un buffer che descrive l'immagine. Ma non è l'immagine stessa.
pub fn take_screenshot(_screenshot_type: Option<ScreenshotType>) -> Option<image::RgbaImage> {
    let mut img;

        // TODO
        // Virtual desktops don't count as screens
        let screen_images = Screen::all().unwrap()
            .into_iter()
            .map(|screen| {
                let image = screen.capture().unwrap();
                ScreenImage{screen, image}
            })
            .collect::<Vec<ScreenImage>>();
        let x_min = screen_images
            .iter()
            .map(|s| s.screen.display_info.x)
            .min()
            .unwrap();
        let y_min = screen_images
            .iter()
            .map(|s| s.screen.display_info.y)
            .min()
            .unwrap();
        let x_max = screen_images
            .iter()
            .map(|s| s.screen.display_info.x + s.screen.display_info.width as i32)
            .max()
            .unwrap();
        let y_max = screen_images
            .iter()
            .map(|s| s.screen.display_info.y + s.screen.display_info.height as i32)
            .max()
            .unwrap();

        
        let offset = (x_min, y_min);
        let size = ((x_max - x_min) as u32, (y_max - y_min) as u32);
        println!("Total screenshot size: {:?}", size);
        println!("Offset: {:?}", offset);

        img = RgbaImage::new(size.0, size.1);
        for screen_image in screen_images {
    
            println!("Screen: {:?}", screen_image.screen.display_info.id);
    
            let mut screenshot = image::io::Reader::new(Cursor::new(screen_image.image.to_png().unwrap()))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
                if screen_image.screen.display_info.scale_factor != 1.0{
                    println!("Scale factor: {}", screen_image.screen.display_info.scale_factor);
                    let scaled_screenshot = screenshot.resize(
                        (screenshot.width() as f32 / screen_image.screen.display_info.scale_factor) as u32,
                        (screenshot.height() as f32 / screen_image.screen.display_info.scale_factor) as u32,
                        image::imageops::FilterType::Nearest
                    );
                    screenshot = scaled_screenshot;
                }
    
                let x = (screen_image.screen.display_info.x - offset.0) as u32;
                let y = (screen_image.screen.display_info.y - offset.1) as u32;
                
                if x + screenshot.width() <= img.width() && y + screenshot.height() <= img.height() {
                    match img.copy_from(&screenshot, x, y) {
                        Ok(_) => (),
                        Err(e) => println!("Failed to copy screen image: {}", e),
                    }
                } else {
                    println!("Screen image is out of bounds: {:?}", screen_image.screen);
                }
        }
        /*for screen_image in all_screen_screenshoots {
            let tmp_screenshot = screen_image.capture().unwrap();
            // screenshot = ImageBuffer::from(prova)
            // TODO -- Temporanea perché per ora considera un solo schermo
            // In realtà screenshot conterrà l'immagine unita di tutti gli screenshot di tutti gli schermi
            screenshot_image_buffer = image::RgbaImage::from(tmp_screenshot);
        }*/

    // TODO gestisci meglio questa situazione: fai che sia una Option<,> prima, così vedi se è andato a buon fine ritorna solo con esito positivo

    return Some(img);
}
