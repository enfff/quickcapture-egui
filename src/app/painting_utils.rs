// Inspired from
// https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/painting.rs

use egui::emath;
use egui::widgets::DragValue;
use egui::{Pos2, Rect, Vec2};
use image::{imageops, RgbaImage};
use std::{ops::Add, vec};

use super::crop_utils;

#[derive(Clone)]

pub struct DrawObj {
    points: Vec<Pos2>,
    stroke: egui::Stroke,
}

impl DrawObj {
    fn new(points: Vec<Pos2>, stroke: egui::Stroke) -> Self {
        Self {
            points: points,
            stroke: stroke,
        }
    }
}

impl Default for DrawObj {
    fn default() -> Self {
        Self {
            points: vec![],
            stroke: egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(18, 160, 215, 255),
            ),
        }
    }
}

#[derive(Clone)]
pub struct Painting {
    /// in 0-1 normalized coordinates
    texture: Option<egui::TextureHandle>,
    pub shapes: Vec<DrawObj>,
    stroke: egui::Stroke,
    aspect_ratio: f32,
    pub screenshot_image_buffer: Option<RgbaImage>,
    last_actions: Vec<DrawObj>, // Used to go back in time!
    ui_size: egui::Rect,
    ui_position: egui::Pos2,
    selected_shape: DrawingShape,
    to_screen: egui::emath::RectTransform,
    crop: Option<crop_utils::Crop>,
    active_shape: bool,
    original_size: (u32, u32),
}

#[derive(Clone, Debug, PartialEq)]
enum DrawingShape {
    Line,
    StraightLine,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            // shapes: Default::default(),
            shapes: vec![],
            stroke: egui::Stroke::new(
                3.0,
                egui::Color32::from_rgba_unmultiplied(18, 160, 215, 255),
            ),
            // https://teamcolorcodes.com/napoli-color-codes/
            texture: None,
            screenshot_image_buffer: None,
            aspect_ratio: 1.,
            last_actions: vec![],
            ui_size: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::ZERO),
            ui_position: egui::Pos2::ZERO,
            selected_shape: DrawingShape::Line,
            crop: None,
            to_screen: emath::RectTransform::identity(Rect::NOTHING),
            active_shape: true,
            original_size: (0, 0),
        }
    }
}

impl Painting {
    pub fn new(
        texture: Option<egui::TextureHandle>,
        screenshot_image_buffer: Option<RgbaImage>,
    ) -> Self {
        Self {
            texture: texture.clone(),
            aspect_ratio: texture.unwrap().aspect_ratio(),
            screenshot_image_buffer: screenshot_image_buffer.clone(),
            original_size: (screenshot_image_buffer.clone().unwrap().width(), screenshot_image_buffer.clone().unwrap().height()),
            ..Self::default()
        }
    }

    pub fn new_crop(
        texture: Option<egui::TextureHandle>,
        screenshot_image_buffer: Option<RgbaImage>,
        shapes: Vec<DrawObj>,
    ) -> Self {
        Self {
            texture: texture.clone(),
            aspect_ratio: texture.unwrap().aspect_ratio(),
            screenshot_image_buffer: screenshot_image_buffer,
            shapes: shapes,
            ..Self::default()
        }
    }

    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if self.texture.is_some() && self.crop.is_none() {
                // Color and stroke buttons
                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut self.stroke.width)
                            .speed(1)
                            .clamp_range(0..=12),
                    )
                    .on_hover_text("Width");
                    ui.color_edit_button_srgba(&mut self.stroke.color);
                    // ui.color_edit_button_srgb(&mut [self.stroke.color.r(), self.stroke.color.g(), self.stroke.color.b()]); // Magheggio per ignorare la trasparenza
                    ui.label("Stroke");

                    egui::ComboBox::from_label("Shape:")
                        .selected_text(format!("{:?}", self.selected_shape))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.selected_shape,
                                DrawingShape::Line,
                                "Line",
                            );
                            ui.selectable_value(
                                &mut self.selected_shape,
                                DrawingShape::StraightLine,
                                "Straight line",
                            );
                        });

                    match self.selected_shape {
                        DrawingShape::Line => {
                            // stroke preview:
                            let (_id, stroke_rect) = ui.allocate_space(ui.spacing().interact_size);
                            let left = stroke_rect.left_center();
                            let right = stroke_rect.right_center();
                            ui.painter().line_segment(
                                [left, right],
                                (*&mut self.stroke.width, *&mut self.stroke.color),
                            );
                        }
                        DrawingShape::StraightLine => {
                            ui.label("StraightLine");
                            let (_id, stroke_rect) = ui.allocate_space(ui.spacing().interact_size);
                            ui.painter().rect_stroke(
                                stroke_rect,
                                egui::Rounding::none(),
                                *&mut self.stroke,
                            );
                        }
                    }
                    if ui.button("✂").clicked() {
                        self.active_shape = false;
                        self.crop = Some(crop_utils::Crop::new());
                    }
                });

                ui.separator();

                if ui.button("Clear Painting").clicked() {
                    self.last_actions = self.shapes.clone();
                    self.shapes.clear();
                }

                ui.separator();

                if self.shapes.is_empty() {
                    self.shapes.push(DrawObj::new(vec![], self.stroke.clone()));
                }

                // UNDO BUTTON
                if self.shapes.is_empty() {
                    if ui
                        .add_enabled(false, egui::Button::new("Undo"))
                        .on_hover_text("Can't go back anymore!")
                        .clicked()
                    {
                        unreachable!();
                    }
                } else if self.shapes.len() > 1 {
                    // shapes will ALWAYS contain an empty vector, which is placed at the end of the array.
                    if ui.button("Undo").clicked() {
                        // Quick and dirty :clown:
                        self.shapes.pop();
                        self.last_actions.pop();
                        

                        self.last_actions.push(self.shapes.pop().unwrap());

                        self.shapes.push(DrawObj::new( vec![], self.stroke));
                        self.last_actions.push(DrawObj::new( vec![], self.stroke));

                        // println!("(after) shapes: {:?}", self.shapes);
                        // println!( "(after) Last actions: {:?}", self.last_actions);
                    }
                }

                ui.separator();

                // REDO BUTTON
                if self.last_actions.is_empty() {
                    if ui
                        .add_enabled(false, egui::Button::new("Redo"))
                        .on_hover_text("Can't go forward")
                        .clicked()
                    {
                        unreachable!();
                    }
                } else if self.last_actions.len() > 1 {
                    // Last_actions will ALWAYS contain an empty vector, which is placed at the end of the array.
                    if ui.button("Redo").clicked() {
                        // Quick and dirty :clown:
                        self.shapes.pop();
                        self.last_actions.pop();

                        self.shapes.push(self.last_actions.pop().unwrap());

                        self.shapes.push(DrawObj::new( vec![], self.stroke));
                        self.last_actions.push(DrawObj::new( vec![], self.stroke));
                    }
                }

                ui.separator();
            } else if self.crop.is_some() {
                //Analizzare qua
                if ui.button("✔").clicked() {
                    let cutrect = self.crop.clone().unwrap().get_cut_rect(Vec2::new(
                        self.screenshot_image_buffer.clone().unwrap().width() as f32,
                        self.screenshot_image_buffer.clone().unwrap().height() as f32,
                    ));
                    let result = imageops::crop(
                        &mut self.screenshot_image_buffer.clone().unwrap(),
                        cutrect.min.x.round() as u32,
                        cutrect.min.y.round() as u32,
                        cutrect.size().x.round() as u32,
                        cutrect.size().y.round() as u32,
                    )
                    .to_image();
                    println!(
                        "Image after cut: {:?} - {:?}",
                        result.width(),
                        result.height()
                    );
                    self.screenshot_image_buffer = Some(result);
                    self.painting_size(ui.available_size());
                    if self.shapes.len() > 0 {
                        self.shapes_remap(&self.crop.clone().unwrap());
                        self.original_size = (self.screenshot_image_buffer.clone().unwrap().width(), self.screenshot_image_buffer.clone().unwrap().height());
                    }
                    self.active_shape = true;
                    self.crop = None;
                } else if ui.button("✖").clicked() {
                    self.active_shape = true;
                    self.crop = None;
                }
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) -> egui::Response {
        // Ritorna una egui::Response, cioè l'esito dell'aggiunta di un widget nella ui. Per farlo,
        // prima crea una UI che mostra lo screenshot come sfondo di un oggetto painter, cioè un
        // livello su cui disegni. Poi, cattura il movimento del mouse per permettere di disegnare,
        // e infine mappa i disegni che hai fatto sopra l'oggetto painter.

        // Bisogna ancora copiare i disegni sopra lo screenshot, covnertirlo in un formato opportuno
        // e salvarlo

        if self.shapes.is_empty() {
            self.shapes.push(DrawObj::new( vec![], self.stroke));
        }

        let painting_size = self.painting_size(ui.available_size());

        // Alloca un oggetto Painter che disegna soltanto in un rettangolo di dimensione painting_size

        // Biggest size possible for the painting by keeping the ar intact

        let (mut response, painter) =
            ui.allocate_painter(painting_size.clone(), egui::Sense::drag());
        self.ui_size = response.rect;
        self.ui_position = response.rect.min;

        // Shows the image we're drawing on
        painter.add(egui::Shape::image(
            self.texture.as_ref().unwrap().id(),
            egui::Rect::from_min_size(response.rect.min, painting_size.clone()), // StraightLine containing the image
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1., 1.)), // uv should normally be Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)) unless you want to crop or flip the image. --> no clue
            egui::Color32::WHITE,
        ));

        self.to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

        let from_screen = self.to_screen.inverse();

        if self.shapes.is_empty() {
            self.shapes.push(DrawObj::new( vec![], self.stroke));
        }
        if self.active_shape == true {
            match self.selected_shape {
                DrawingShape::Line => {
                    let current_line = self.shapes.last_mut().unwrap();
                    if let Some(pointer_pos) = response.interact_pointer_pos() {
                        let canvas_pos = from_screen * pointer_pos;

                        // println!("Canvas pos: {:?}", canvas_pos);
                        if current_line.points.last() != Some(&canvas_pos) {
                            if current_line.stroke != self.stroke {
                                current_line.stroke = self.stroke;
                            }
                            current_line.points.push(canvas_pos);
                            response.mark_changed();
                        }
                    } else if !current_line.points.is_empty() {
                        self.shapes.push(DrawObj::new( vec![], self.stroke));
                        response.mark_changed();
                    }
                }
                DrawingShape::StraightLine => {
                    let current_line = self.shapes.last_mut().unwrap();
                    let init_canvas_pos: Pos2;
                    let mut next_canvas_pos = egui::Pos2::new(-1., -1.); // This is the rectange bottom right point as we're dragging it

                    if response.clicked() {
                        if let Some(pointer_pos) = response.interact_pointer_pos() {
                            // La prima volta che viene premuto cattura la posizione iniziale
                            init_canvas_pos = from_screen * pointer_pos;
                            println!("Init canvas pos: {:?}", init_canvas_pos);
                        }

                        if let Some(pointer_pos) = response.hover_pos() {
                            // L'utente sta spostando il mouse nell'immagine -> disegna anteprima
                            // current_line.pop(); // Elimina ultima linea disegnata

                            println!("Hover pos: {:?}", pointer_pos);

                            if current_line.points.last() != Some(&next_canvas_pos) {
                                next_canvas_pos = from_screen * pointer_pos;
                                current_line.points.push(next_canvas_pos);
                                response.mark_changed();
                            }

                            // current_line.pop();
                        }

                        if response.clicked() {
                            // L'utente ha rilasciato il mouse -> disegna la linea
                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.points.last() != Some(&next_canvas_pos) {
                                    next_canvas_pos = from_screen * pointer_pos;
                                    current_line.points.push(next_canvas_pos);
                                    response.mark_changed();
                                }
                            }
                        }
                    }
                }
            }
        }
        // Ridisegna le linee
        let shapes = self
            .shapes
            .iter()
            .filter(|line| line.points.len() >= 2)
            .map(|line| {
                let points: Vec<egui::Pos2> = line.points.iter().map(|p| self.to_screen * *p).collect();
                egui::Shape::line(points, line.stroke)
            });
        painter.extend(shapes);

        if self.crop.is_some() {
            self.crop.as_mut().unwrap().crop_img(
                ui,
                response.clone(),
                egui::Vec2::new(self.ui_size.width(), self.ui_size.height())
            );
        }
        response
    }

    pub fn generate_rgba_image(&mut self) -> RgbaImage {
        //  Prende tutte le shapes fatte, che sono composte da coordinate (egui::Pos2)
        let mut output_image = self.screenshot_image_buffer.clone();

        println!("{:?}", self.stroke.width);

        // let from_screen = to_screen.inverse();

        // Ho dovuto clonare perché altrimenti dava problemi il borrow checker
        for line in self.shapes.clone().iter() {
            for couple_points in line.points.windows(2) {
                for offset in 0..= line.stroke.width as u8 {
                    // Domanda: Perché è stato fatto così?
                    // Risposta: Egui può disegnare le linee con un certo spessore perché per ogni coppia di punti disegna un rettangolo con uno spessore.
                    // Ho cercato di fare lo stesso con imageproc, ma non si può fare perché la linea non ha un argomento per lo spessore. Ho cercato di farlo
                    // manualmente traslando di vari offset, ma uscivano sempre dei buchi bianchi. Ho cercato di riempirli, e attualmente questo è stato il risultato migliore
                    // Non posso perderci altro tempo. La libreria è troppo acerba.

                    let mut start = self.segment_coordinates(&couple_points[0], (offset, offset));
                    let mut end = self.segment_coordinates(&couple_points[1], (offset, offset));

                    imageproc::drawing::draw_line_segment_mut(
                        output_image.as_mut().unwrap(),
                        start,
                        end,
                        image::Rgba(line.stroke.color.to_array()),
                    );
                    // let rect = imageproc::rect::RectPosition::

                    start = self.segment_coordinates(&couple_points[0], (0, offset));
                    end = self.segment_coordinates(&couple_points[1], (0, offset));
                    imageproc::drawing::draw_line_segment_mut(
                        output_image.as_mut().unwrap(),
                        start,
                        end,
                        image::Rgba(line.stroke.color.to_array()),
                    );

                    start = self.segment_coordinates(&couple_points[0], (offset, 0));
                    end = self.segment_coordinates(&couple_points[1], (offset, 0));
                    imageproc::drawing::draw_line_segment_mut(
                        output_image.as_mut().unwrap(),
                        start,
                        end,
                        image::Rgba(line.stroke.color.to_array()),
                    );
                    // imageproc::drawing::draw_filled_rect_mut(output_image.as_mut().unwrap(), start, end);
                }
            }
        }

        return output_image.unwrap().clone();
    }

    /*     pub fn ui_size(&mut self) -> egui::Rect{
        // Ritorna la grandezza della UI, usata per il cropping
        return self.ui_size
    }

    pub fn ui_size_vec2(&mut self) -> egui::Vec2{
        // Ritorna la grandezza della UI, usata per il cropping
        return egui::Vec2::new(self.ui_size.width(), self.ui_size.height())
    }

    pub fn ui_position(&mut self) -> egui::Pos2{
        // Ritorna la posizione della UI, usata per il cropping
        return self.ui_position
    }*/

    fn segment_coordinates(&mut self, point: &egui::Pos2, offset: (u8, u8)) -> (f32, f32) {
        // Trasforma le coordinate di un punto p che è stato normalizzato con proporzioni quadrate, in coordinate per l'immagine

        let w = self.screenshot_image_buffer.as_ref().unwrap().width();
        let h = self.screenshot_image_buffer.as_ref().unwrap().height();

        let output_size = egui::Vec2::new(w as f32, h as f32);
        let rect_output_size = egui::Rect::from_min_size(egui::Pos2::ZERO, output_size);

        // Serve per trasformare i punti p di shapes, in coordinate per lo schermo
        self.to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_min_size(egui::Pos2::ZERO, rect_output_size.square_proportions()),
            rect_output_size,
        );

        // .add(egui::Vec2::new(self.stroke.width/2. + offset as f32, self.stroke.width/2. + offset as f32))

        let mut new_coordinates = self.to_screen * *point;

        // Add an offset to simulate the width
        new_coordinates = new_coordinates.add(egui::Vec2::new(
            self.stroke.width / 2. + offset.0 as f32,
            self.stroke.width / 2. + offset.1 as f32,
        ));

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

        return [new_coordinates.x, new_coordinates.y].into();
    }

    fn painting_size(&mut self, ui_available_size: egui::Vec2) -> egui::Vec2 {
        // Definisce la grandezza dell'immagine su cui stai disegnando. Prende sempre la grandezza minore
        // tra la grandezza della UI e quella dello screenshot, mantendo intatto l'aspect ratio.

        // TODO: caso in cui image è TALL

        let mut painting_size = egui::Vec2::ZERO;
        if ui_available_size.x < ui_available_size.y && self.aspect_ratio >= 1. {
            // Image is FAT, and x < y
            painting_size =
                egui::Vec2::from([ui_available_size.x, ui_available_size.x / self.aspect_ratio]);
        } else if ui_available_size.x > ui_available_size.y {
            // Image is FAT, and x >= y
            painting_size =
                egui::Vec2::from([ui_available_size.y * self.aspect_ratio, ui_available_size.y]);
        };

        return painting_size;
    }

    pub fn shapes_remap(&mut self, crop: &crop_utils::Crop) { 
        // Remaps the shapes coordinates to the new image size
        for shape in &mut self.shapes {
            let points: Vec<Pos2> = shape.points.iter_mut().map(|p| {
                let mut point =  *p;
                point.x -= crop.offset_x_left;
                point.y -= crop.offset_y_up;

                return point;
            }).collect();

            //println!("--------------------------------");

            shape.points = points;
        }
    }
}
