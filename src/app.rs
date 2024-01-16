use image::{ImageBuffer, RgbaImage};
use std::sync::mpsc;
use std::{thread, time};

mod image_utils;
mod painting_utils;
mod pathlib;
mod save_utils;
mod screenshot_utils;
mod crop_lib;

use crate::app::save_utils::SavePath;

pub enum Views {
    Home,
    Settings,
    Capture,
    Save,
    Crop
}

#[derive(Clone, Eq, PartialEq)]
pub enum ScreenshotType {
    FullScreen,
    PartialScreen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImgFormats {
    PNG,
    JPEG,
    GIF,
}

pub struct QuickCaptureApp {
    pub view: Views,
    screenshot_image_buffer: Option<RgbaImage>, // The screenshot data
    screenshot_type: Option<ScreenshotType>,
    painting: Option<painting_utils::Painting>, // UI and methods to draw on the screenshot
    painted_screenshot: Option<egui::TextureHandle>, // egui wants TextureHandles for painting on things. However, this cannot be used to save the image.
    timer_delay: u64,
    crop_rectangle: Option<egui::Rect>,
    pub save_path: SavePath,
}

impl Default for QuickCaptureApp {
    fn default() -> Self {
        Self {
            view: Views::Home,
            screenshot_type: None,
            screenshot_image_buffer: None, // https://teamcolorcodes.com/napoli-color-codes/
            painting: None,
            crop_rectangle: None,
            painted_screenshot: None,
            save_path: SavePath::new(std::env::current_dir().unwrap().join("target"), ImgFormats::PNG),         // Salva in <app_directory>/target/
            timer_delay: 0,
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl QuickCaptureApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    // Views (the current view)
    pub fn home_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(
            ctx,
            |ui| {
                // Some emojis can be used as icons!
                // https://docs.rs/egui/0.24.1/egui/special_emojis/index.html
                // https://www.egui.rs/#demo (Font Book)

                ui.horizontal(|ui| {
                    if ui.small_button("📷 Take Screenshot").clicked() {
                        println!("Screenshot button pressed");
                        self.view = Views::Capture;
                    }

                    if self.screenshot_image_buffer.is_some(){
                        // Se è stato fatto uno screenshot, mostra i bottoni per aggiungere modifiche e salvarlo

                        ui.separator();
                        if ui.small_button("💾 Save").clicked() {
                            println!("Save button pressed");
                            self.view = Views::Save;
                        }

                    }

                    ui.separator();
                    if ui.small_button("Settings").clicked() {
                        println!("Settings button pressed");
                        self.view = Views::Settings;
                    }
                });

                if self.screenshot_image_buffer.is_none() {
                    // Ancora non è stato fatto alcuno screenshot
                    ui.centered_and_justified(|ui| ui.label("Take a screenshot"));
                } else {
                    // È stato fatto uno screenshot -> mostralo e se vuoi disegnaci
                    ui.vertical_centered(|ui| {
                        if self.screenshot_image_buffer.is_some() {
                            if self.painting.is_none() {
                                self.painted_screenshot = Some(ui.ctx().load_texture(
                                    "painted_screenshot",
                                    image_utils::load_image_from_memory(
                                        self.screenshot_image_buffer.clone().unwrap(),
                                    ),
                                    Default::default(),
                                ));

                                // Create an istance of a Painter object
                                self.painting = Some(painting_utils::Painting::new(
                                    self.painted_screenshot.clone(),
                                    self.screenshot_image_buffer.clone(),
                                ));
                            }

                            let painting = self.painting.as_mut().unwrap();

                            // Aggiunge i controlli per disegnare (linea, cerchio, quadrato, ecc...)
                            painting.ui_control(ui);
                            // Aggiunge un livello che ha come sfondo lo screenshot su cui sopra è possibile disegnare
                            painting.ui_content(ui);

                            self.painting = Some(painting.clone());
                        };
                    });
                }
            }, // Central Panel
        );
    }

    pub fn settings_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Will contain the shortcuts
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Settings view");
            if ui.button("go back").clicked() {
                self.view = Views::Home;
            };

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
                ui.add(egui::github_link_file!(
                    "https://github.com/enfff/quickcapture-egui/blob/master/",
                    "Source code"
                ));
            });
        });
    }

    pub fn screenshot_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // println!("screenshot_view");
        
        // Prima hai scelto che screenshot fare, adesso fai lo screenshot
        // Questa parte è stata anticipata altrimenti si vedrebbe la maschera disegnata nelle righe successive
        if self.screenshot_type.is_some() {
            // It's not the screenshot, but the data describing it. It needs to be converted to an image.

            // quick and dirty solution, not too proud but i couldn't find any  other way around it...
            thread::sleep(time::Duration::from_millis(&self.timer_delay + 150)); // 150 always needed to compensate EGUI's lag

            let (tx_screenshot_buffer, rx_screenshot_buffer) = mpsc::channel::<Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>>();
            let tmp_screenshot_type = self.screenshot_type.clone();

            // Take the screenshot and wait until it's done
            thread::spawn(move || {
                let screenshot_image_buffer = screenshot_utils::take_screenshot(tmp_screenshot_type);
                tx_screenshot_buffer.send(screenshot_image_buffer).unwrap();
            });

            self.screenshot_image_buffer = rx_screenshot_buffer.recv().unwrap();
            self.save_path.name = save_utils::generate_filename();
            println!("default filename is: {}", self.save_path.name);
            self.view = Views::Home;
            self.screenshot_type = None;
            self.painting = None;
        }

        // Mostra UI per generare screenshot

        // Se l'utente non ha scelto che tipo di screenshot fare (tra FullScreen e PartialScreen)
        // Questa funzione viene chiamata ad ogni update() che può avvenire più volte in un secondo.
        // Siccome noi vogliamo che lo per chiamata di "screenshot_view", si fa un controllo con un contatore

        if self.screenshot_type.is_none() {
            _frame.set_visible(true);

            // Maschera sopra lo schermo per scegliere il tipo di screenshot
            egui::Window::new("screenshot_view")
                .title_bar(false)
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    // self.id = Some(ui.layer_id());
                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {

                            if ui.button("Go back").clicked() {
                                self.view = Views::Home;
                            }
                            ui.separator();

                            if ui.button("⛶ Partial").clicked() {
                                self.screenshot_type = Some(ScreenshotType::PartialScreen);
                                println!("PartialScreen button pressed");
                            }
                            ui.separator();

                            if ui.button("🖵 Full").clicked() {
                                self.screenshot_type = Some(ScreenshotType::FullScreen);
                                println!("FullScreen button pressed");
                            }
                            ui.separator();

                            if self.screenshot_type.is_some() {
                                // L'utente ha scelto che screenshot da fare
                                println!("screenshot_type is some");

                                // Hides the screen
                                _frame.set_visible(false);
                                ui.set_visible(false);
                                ctx.request_repaint();
                            }

                            ui.add(egui::DragValue::new(&mut self.timer_delay).speed(50).max_decimals(2).clamp_range(0..=10000).prefix("Delay Timer (ms): "));

                        });
                    });
                });
        }
    }

    pub fn save_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // println!("settings_view");
            pathlib::ui(ui, &mut self.save_path);
            if ui.button("Save").clicked() {
                println!("Save button pressed");

                save_utils::save_image(
                    &self.save_path,
                    self.painting.as_mut().unwrap().generate_rgba_image(),
                );
                self.view = Views::Home;
            };
            if ui.button("Go back").clicked() {
                self.view = Views::Home;
            };
        });
    }

    pub fn crop_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui| {
            println!("Crop");
        });
    }
}