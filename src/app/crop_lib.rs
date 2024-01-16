use crate::app::painting_utils::Painting;

pub struct Crop {
    new_size: egui::Vec2,
    painting: Option<Painting>,
}

impl Default for Crop {
    fn default() -> Self {
        Self {
            new_size: egui::Vec2::ZERO,
            painting: None,
        }
    }
}

impl Crop {
    pub fn new(painting: Option<Painting>) -> Self {
        Self {
            painting: painting,
            ..Self::default()
        }
    }
}