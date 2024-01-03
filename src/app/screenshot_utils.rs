use screenshots::Screen;
use crate::app::ScreenshotType;

// Fa lo screenshot di ogni schermo e ritorna un buffer che descrive l'immagine. Ma non è l'immagine stessa. 
pub fn take_screenshot(format: &str, mut screenshot_type: Option<ScreenshotType>) -> Option<image::RgbaImage> {
    let mut screenshot_image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::RgbaImage::new(1920, 1080);

    // Virtual desktops don't count as screens
    let all_screen_screenshoots = Screen::all().unwrap();

    for screen_image in all_screen_screenshoots {
        let mut tmp_screenshot = screen_image.capture().unwrap();
        // screenshot = ImageBuffer::from(prova)
        // TODO -- Temporanea perché per ora considera un solo schermo
        // In realtà screenshot conterrà l'immagine unita di tutti gli screenshot di tutti gli schermi
        screenshot_image_buffer = image::RgbaImage::from(tmp_screenshot);
    }

    // TODO gestisci meglio questa situazione: fai che sia una Option<,> prima, così vedi se è andato a buon fine ritorna solo con esito positivo
    
    return Some(screenshot_image_buffer);
}