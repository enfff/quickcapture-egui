// Inspired from
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/painting.rs

// Should

#[derive(Clone)]
pub struct Painting {
    /// in 0-1 normalized coordinates
    lines: Vec<Vec<egui::Pos2>>,
    stroke: egui::Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 160, 215)),
            // https://teamcolorcodes.com/napoli-color-codes/
        }
    }
}

impl Painting {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
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
    pub fn ui_content(&mut self, ui: &mut egui::Ui, where_i_am_painting: &egui::TextureHandle) -> egui::Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());

        let to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        painter.add(egui::Shape::image(
            where_i_am_painting.id(),
            egui::Rect::from_min_size(response.rect.min, egui::vec2(1920., 1080.)),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1., 1.)),
            egui::Color32::WHITE)
        );

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
}