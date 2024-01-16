use egui::*;
use image::RgbaImage;
use std::sync::mpsc;
use std::{thread, time};

mod image_utils;
mod painting_utils;
mod pathlib;
mod save_utils;
mod screenshot_utils;
mod screenshot_view;
mod crop_lib;

use crate::app::save_utils::SavePath;

use self::save_utils::check_filename;

pub enum Views {
    Home,
    Settings,
    Capture,
    Save,
    Crop
}

#[derive(Clone, Eq, PartialEq, Debug)]
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
    screenshot_view: screenshot_view::ScreenshotView,
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
            save_path: SavePath::new(
                std::env::current_dir().unwrap().join("target"),
                ImgFormats::PNG,
            ), // Salva in <app_directory>/target/
            timer_delay: 0,
            screenshot_view: screenshot_view::ScreenshotView::new(),
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
        self.screenshot_view.ui(ctx, _frame, &mut self.view, &mut self.screenshot_type);
        if self.screenshot_type.is_some() {
            // It's not the screenshot, but the data describing it. It needs to be converted to an image.
            _frame.set_window_size(vec2(640.0, 400.0));
            _frame.set_centered();
            _frame.set_visible(false);
            ctx.request_repaint();
            // quick and dirty solution, not too proud but i couldn't find any  other way around it...
            thread::sleep(time::Duration::from_millis(150));
            let (tx_screenshot_buffer, rx_screenshot_buffer) = mpsc::channel();
            let tmp_screenshot_type = self.screenshot_type.clone();
            let ctx1 = ctx.clone();
            if self.screenshot_type.clone().unwrap() == ScreenshotType::FullScreen {
                // Take the screenshot and wait until it's done
                thread::spawn(move || {
                    let screenshot_image_buffer =
                        screenshot_utils::take_screenshot(tmp_screenshot_type, None, &ctx1);
                    tx_screenshot_buffer.send(screenshot_image_buffer).unwrap();
                });
            } else if self.screenshot_type.clone().unwrap() == ScreenshotType::PartialScreen {
                //TODO
                let grab = self.screenshot_view.clone();
                thread::spawn(move || {
                    let screenshot_image_buffer =
                        screenshot_utils::take_screenshot(tmp_screenshot_type, Some(grab), &ctx1);
                    tx_screenshot_buffer.send(screenshot_image_buffer).unwrap();
                });
            }

            self.screenshot_image_buffer = rx_screenshot_buffer.recv().unwrap();

            if self.screenshot_image_buffer.is_some() {
                self.save_path.name = save_utils::generate_filename();
                println!("default filename is: {}", self.save_path.name);
            }
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
            
    }
}

    pub fn save_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // println!("settings_view");
            pathlib::ui(ui, &mut self.save_path);
            let save_button =
                ui.add_enabled(check_filename(&self.save_path.name), Button::new("Save"));
            if save_button.clicked() {
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

