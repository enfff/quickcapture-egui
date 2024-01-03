use image::ImageBuffer;
use std::sync::mpsc::Sender;

pub fn save_image(screenshot_image_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>>, path: &str, tx: Sender<std::time::Duration>){
    // Do something
    let instant = std::time::Instant::now(); // Start timer
    let path2 = "hi.png".to_string();

    tokio::spawn(async move {
        image::save_buffer(path2,  &screenshot_image_buffer.as_raw().as_slice(), 1920, 1080, image::ColorType::Rgba8).unwrap();
        // image::save_buffer(path,  &screenshot_image_buffer.as_raw().as_slice(), 1920, 1080, image::ColorType::Rgba8); // for quick testing
    });

    let duration = instant.elapsed();
    println!("save_image time elapsed: {:?}", duration);
    tx.send(duration).unwrap();
}