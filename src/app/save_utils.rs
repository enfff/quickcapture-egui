use image::ImageBuffer;
use std::sync::mpsc::Sender;

pub fn save_image(picture: ImageBuffer<image::Rgba<u8>, Vec<u8>>, tx: Sender<std::time::Duration>){
    // Do something
    let instant = std::time::Instant::now(); // Start timer

    let path = generate_filename();

    tokio::spawn(async move {
        image::save_buffer(path,  &picture.as_raw().as_slice(), picture.width(), picture.height(), image::ColorType::Rgba8).unwrap();
        // image::save_buffer(path,  &picture.as_raw().as_slice(), 1920, 1080, image::ColorType::Rgba8); // for quick testing
    });

    let duration = instant.elapsed();
    println!("save_image time elapsed: {:?}", duration);
    tx.send(duration).unwrap();
}

pub fn generate_filename() -> String {
    // TODO
    "hi.png".to_string()
}