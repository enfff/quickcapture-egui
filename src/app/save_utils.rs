use image::ImageBuffer;
// use std::sync::mpsc::Sender;
use crate::app::ImgFormats;
use std::path::PathBuf;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavePath{
    pub path: PathBuf,
    pub name: String,
    pub format: ImgFormats,
    pub user_mod_name: bool,
}

impl SavePath {
    pub fn new(path: PathBuf, format: ImgFormats) -> Self {
        let date: DateTime<Local> = Local::now();
        let formatted = date.format("%Y-%m-%dT%H:%M:%S");
        let name = formatted.to_string();
        Self {
            path,
            format,
            name,
            user_mod_name: false,
        }
    }

}

pub fn save_image(save_path: &SavePath, picture: ImageBuffer<image::Rgba<u8>, Vec<u8>>){
    // Questa funzione in base al path selezionato e al nome file, salva l'immagine
    // L'immagine salvata è gestita correttamente da image::save_buffer che salva correttamente nel formato desiderato
    // Ovviamente si presume che il path sia valido e che l'estensione del file sia valida
    let mut pathname = save_path.path.to_string_lossy().to_string().to_owned();
    pathname.push_str("/");
    let filename = save_path.name.to_owned();
    pathname.push_str(&filename);
    if save_path.format == ImgFormats::PNG{
        pathname.push_str(".png");
    }
    else if save_path.format == ImgFormats::JPEG{
        pathname.push_str(".jpeg");
    }
    else if save_path.format == ImgFormats::GIF{
        pathname.push_str(".gif");
    }
    println!("Saving image to {}", pathname);

    image::save_buffer(pathname,  &picture.as_raw().as_slice(), picture.width(), picture.height(), image::ColorType::Rgba8).unwrap();
}

pub fn generate_filename() -> String {
    let date: DateTime<Local> = Local::now();
    let formatted = date.format("%Y-%m-%dT%H_%M_%S");
    formatted.to_string()
}