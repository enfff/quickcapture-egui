// Inspired from
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/painting.rs

// Should

use egui::output;
use image::{RgbaImage, Pixel};
use imageproc::drawing::{Canvas, self};

#[derive(Clone)]
pub struct Painting {
    /// in 0-1 normalized coordinates
    texture: Option<egui::TextureHandle>,   
    lines: Vec<Vec<egui::Pos2>>,
    stroke: egui::Stroke,
    aspect_ratio: f32,
    screenshot_image_buffer: Option<RgbaImage>,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 160, 215)),
            // https://teamcolorcodes.com/napoli-color-codes/
            texture: None,
            screenshot_image_buffer: None,
            aspect_ratio: 1.,
        }
    }
}

impl Painting {
    pub fn new(texture: Option<egui::TextureHandle>, screenshot_image_buffer: Option<RgbaImage>) -> Self {
        Self {
            texture: texture.clone(),
            aspect_ratio: texture.unwrap().aspect_ratio(),
            screenshot_image_buffer: screenshot_image_buffer,
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
    pub fn ui_content(&mut self, ui: &mut egui::Ui) -> egui::Response {
        // Ritorna una egui::Response, cioè l'esito dell'aggiunta di un widget nella ui. Per farlo,
        // prima crea una UI che mostra lo screenshot come sfondo di un oggetto painter, cioè un
        // livello su cui disegni. Poi, cattura il movimento del mouse per permettere di disegnare, 
        // e infine mappa i disegni che hai fatto sopra l'oggetto painter.
        
        // Bisogna ancora copiare i disegni sopra lo screenshot, covnertirlo in un formato opportuno
        // e salvarlo

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let painting_size = self.painting_size(ui.available_size());
    
        // Alloca un oggetto Painter che disegna soltanto in un rettangolo di dimensione painting_size
        
        // REDO - Funziona ma potrebbe andare meglio
        // Biggest size possible for the painting by keeping the ar intact

        
        let (mut response, painter) = ui.allocate_painter(painting_size.clone(), egui::Sense::drag());

        // Shows the image we're drawing on
        painter.add(egui::Shape::image(
            self.texture.as_ref().unwrap().id(),
            egui::Rect::from_min_size(response.rect.min, painting_size.clone()),                          // Rectangle containing the image
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1., 1.)),         // uv should normally be Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)) unless you want to crop or flip the image. --> no clue
            egui::Color32::WHITE)
        );
        
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

            // println!("Canvas pos: {:?}", canvas_pos);
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
        //  Prende tutte le shapes fatte, che sono composte da coordinate (egui::Pos2)
        let mut output_image = self.screenshot_image_buffer.clone();

        // let from_screen = to_screen.inverse();
        
        // Ho dovuto clonare perché altrimenti dava problemi il borrow checker
        for line in self.lines.clone().iter() {
            for couple_points in line.windows(2) {

                let start = self.segment_coordinates(&couple_points[0]);
                let end = self.segment_coordinates(&couple_points[1]);

                // output_image.as_mut().unwrap().put_pixel(new_coordinates.x as u32, new_coordinates.y as u32, translucent_red);
                // output_image.as_mut().unwrap().draw_line_segment_mut(start_coordinates, end_coordinates, translucent_red);

                // let mut prova = output_image.as_mut().unwrap().get_pixel(start.0 as u32, start.1 as u32);

                imageproc::drawing::draw_line_segment_mut(output_image.as_mut().unwrap(), start, end, image::Rgba(self.stroke.color.to_array()));
            }
        }


        return output_image.unwrap().clone();
    }



    fn segment_coordinates(&mut self, point: &egui::Pos2) -> (f32, f32){
        // Trasforma le coordinate di un punto p che è stato normalizzato con proporzioni quadrate, in coordinate per l'immagine

        let w = self.screenshot_image_buffer.as_ref().unwrap().width();
        let h = self.screenshot_image_buffer.as_ref().unwrap().height();

        let output_size = egui::Vec2::new(w as f32, h as f32);
        let rect_output_size = egui::Rect::from_min_size(egui::Pos2::ZERO, output_size);

        // Serve per trasformare i punti p di shapes, in coordinate per lo schermo
        let to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_min_size(egui::Pos2::ZERO, rect_output_size.square_proportions()),
            rect_output_size,
        );

        let mut new_coordinates = to_screen * *point;

        if new_coordinates.x > output_size.x {
            // If it's out the screen, then something went wrong
            new_coordinates.x = output_size.x - 1.;
        } else if new_coordinates.x < 0. {
            new_coordinates.x = 0.;
        }

        if new_coordinates.y > output_size.y {
            // If it's out the screen, then something went wrong
            new_coordinates.y = output_size.y - 1.;
        } else if new_coordinates.y < 0. {
            new_coordinates.y = 0.;
        }

        return [new_coordinates.x, new_coordinates.y].into()
    }

    fn painting_size(&mut self, ui_available_size: egui::Vec2) -> egui::Vec2 {
        // Definisce la grandezza dell'immagine su cui stai disegnando. Prende sempre la grandezza minore
        // tra la grandezza della UI e quella dello screenshot, mantendo intatto l'aspect ratio.

        let mut painting_size =  egui::Vec2::ZERO;
        if ui_available_size.x < ui_available_size.y && self.aspect_ratio >= 1.{
            // Image is FAT, and x < y
            painting_size = egui::Vec2::from([ui_available_size.x, ui_available_size.x/self.aspect_ratio]);
        } else if ui_available_size.x > ui_available_size.y {
            // Image is FAT, and x >= y
            painting_size = egui::Vec2::from([ui_available_size.y*self.aspect_ratio, ui_available_size.y]);
        };

        return painting_size
    }
}