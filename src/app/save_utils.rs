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
    // Do something
    let mut pathname = save_path.path.to_string_lossy().to_string().to_owned();
    pathname.push_str("/");
    let filename = save_path.name.to_owned();
    pathname.push_str(&filename);
    pathname.push_str(".png");
    println!("Saving image to {}", pathname);

    image::save_buffer(pathname,  &picture.as_raw().as_slice(), picture.width(), picture.height(), image::ColorType::Rgba8).unwrap();
}

pub fn generate_filename() -> String {
    let date: DateTime<Local> = Local::now();
    let formatted = date.format("%Y-%m-%dT%H:%M:%S");
    formatted.to_string()
}