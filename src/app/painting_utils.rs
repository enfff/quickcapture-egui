// Inspired from
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/painting.rs

// Should

use egui::Vec2;

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
        // Ritorna una egui::Response, cioè l'esito dell'aggiunta di un widget nella ui. Per farlo,
        // prima crea una UI che mostra lo screenshot come sfondo di un oggetto painter, cioè un
        // livello su cui disegni. Poi, cattura il movimento del mouse per permettere di disegnare, 
        // e infine mappa i disegni che hai fatto sopra l'oggetto painter.
        
        // Bisogna ancora copiare i disegni sopra lo screenshot, covnertirlo in un formato opportuno
        // e salvarlo

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let ui_available_size: egui::Vec2 = ui.available_size();                    // Dimensione della UI

        let aspect_ratio = texture.aspect_ratio();                             // Aspect ratio dello screenshot

        // Definisce la grandezza dell'immagine su cui stai disegnando. Prende sempre la grandezza minore
        // tra la grandezza della UI e quella dello screenshot, mantendo intatto l'aspect ratio.
        // Ci sono problemi intrinseci quando la UI è troppo piccola -> dobbiamo assicurari abbia almeno una certa dimensione fissa
        
        // Alloca un oggetto Painter che disegna soltanto in un rettangolo di dimensione desired_size
        
        // REDO - Funziona ma potrebbe andare meglio
        // Biggest size possible for the painting by keeping the ar intact
        let mut painting_size = Vec2::ZERO;

        if ui_available_size.x < ui_available_size.y && aspect_ratio >= 1.{
            // Image is FAT, and x < y
            painting_size = egui::Vec2::from([ui_available_size.x, ui_available_size.x/aspect_ratio]);
        } else if ui_available_size.x > ui_available_size.y {
            // Image is FAT, and x >= y
            painting_size = egui::Vec2::from([ui_available_size.y*aspect_ratio, ui_available_size.y]);
        };

        let (mut response, painter) = ui.allocate_painter(painting_size.clone(), egui::Sense::drag());

        // Shows the image we're drawing on
        painter.add(egui::Shape::image(
            texture.id(),
            egui::Rect::from_min_size(response.rect.min, painting_size.clone()),                          // Rectangle containing the image
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1., 1.)),         // uv should normally be Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)) unless you want to crop or flip the image. --> no clue
            egui::Color32::WHITE)
        );
        
        // Normalizza ad 1
        let to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

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

    pub fn generate_rgba_image(&mut self, texture: &egui::TextureHandle) -> image::RgbaImage{
        //  
        

        
        return image::RgbaImage::new(10, 10);
    }
}