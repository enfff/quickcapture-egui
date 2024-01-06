// Inspired from
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/painting.rs

// Should

#[derive(Clone)]
pub struct Painting {
    /// in 0-1 normalized coordinates
    texture: Option<egui::TextureHandle>,   
    lines: Vec<Vec<egui::Pos2>>,
    stroke: egui::Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 160, 215)),
            // https://teamcolorcodes.com/napoli-color-codes/
            texture: None,
        }
    }
}

impl Painting {
    pub fn new(texture: Option<egui::TextureHandle>) -> Self {
        Self {
            texture: texture,
            ..Self::default()
        }
    }

    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        if self.texture.is_none() {
            println!("Texture is none");
        }

        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
            
            ui.separator();

            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
            }

        })
        .response
    }

    // Trovare un nome migliore per where_i_am_painting
    pub fn ui_content(&mut self, ui: &mut egui::Ui, texture: &egui::TextureHandle) -> egui::Response {

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        // Mostra lo screenshot come sfondo di un canvas, cioè un livello su cui disegni

        let current_size: egui::Vec2 = ui.available_size();
        let aspect_ratio = texture.aspect_ratio();

        
        // Calcola la dimensione del canvas rispetto la dimensione dello screenshot.
        // Questa dimensione viene poi usata per la canvas, e la sua texture
        let mut desired_size: egui::Vec2 = 
        if current_size.min_elem() == current_size.x {
            egui::Vec2::from([current_size.x, current_size.x/aspect_ratio])
        } else {
            egui::Vec2::from([aspect_ratio*current_size.y, current_size.y])

        };

        // See è troppo piccola (<400), rendila lameno 400
        if desired_size.x < 600. {
            println!("Old Desired size: {:?}", desired_size);
            desired_size = egui::Vec2::from([600., 600./aspect_ratio]);
            println!("New Desired size: {:?}", desired_size);
        }
        
        // Alloca un oggetto Painter che disegna soltanto in un rettangolo di dimensione desired_size
        let (mut response, painter) = ui.allocate_painter(desired_size, egui::Sense::drag());

        // Shows the image we're drawing on
        painter.add(egui::Shape::image(
            texture.id(),
            egui::Rect::from_min_size(egui::Pos2::ZERO, desired_size.clone()),                           // Rectangle containing the image
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1., 1.)),            // uv should normally be Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)) unless you want to crop or flip the image. --> no clue
            egui::Color32::WHITE)
        );
        
        let to_screen = egui::emath::RectTransform::from_to(
            // egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.size()),
            egui::Rect::from_min_size(egui::Pos2::ZERO, desired_size.clone()),          // Alloca un rettangolo disegnato da (0,0) di dimensione desired_size
            response.rect,
        );
        // let from_screen = to_screen.inverse();              // Non lo so perché serve
        let from_screen = to_screen.clone();              // Non lo so perché serve

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
            response.mark_changed();
        }

        // Ridisegna le linee
        let shapes = self
            .lines
            .iter()
            .filter(|line| line.len() >= 2)
            .map(|line| {
                let points: Vec<egui::Pos2> = line.iter().map(|p| to_screen * *p).collect();
                egui::Shape::line(points, self.stroke)
            });

        painter.extend(shapes);
        response
    }

    pub fn generate_rgba_image(&mut self) -> image::RgbaImage{
        return image::RgbaImage::new(1920, 1080);
    }
}