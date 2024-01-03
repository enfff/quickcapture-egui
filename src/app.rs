use std::sync::mpsc;
use std::{thread, time};
use image::{RgbaImage, ImageBuffer};

mod screenshot_utils;
mod save_utils;
mod image_utils;

pub enum Views {
    Home,
    Settings,
    Capture,
}

#[derive(Clone)]
pub enum ScreenshotType {
    FullScreen,
    PartialScreen,
}

pub struct QuickCaptureApp {
    pub view: Views,
    // Used by the tokio runtime
    pub tx: mpsc::Sender<std::time::Duration>,
    pub rx: mpsc::Receiver<std::time::Duration>,
    pub is_app_saving: bool,
    texture: Option<egui::TextureHandle>,   // Used to display the screenshot
    screenshot_image_buffer: Option<RgbaImage>,
    screenshot_type: Option<ScreenshotType>,
}

impl Default for QuickCaptureApp {
    fn default() -> Self {

        let (tx, rx) = std::sync::mpsc::channel::<std::time::Duration>();

        // Default options
        Self {
            view: Views::Home,
            tx,
            rx,
            texture: None,
            screenshot_type: None,
            screenshot_image_buffer: None,
            is_app_saving: false,
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's


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
                        if self.screenshot_image_buffer.is_some() {
                            save_utils::save_image(self.screenshot_image_buffer.clone().unwrap(), "screenshot.png", self.tx.clone());
                        }
                    }
                    
                    ui.separator();
                    if ui.small_button("↖ Arrow").clicked() {
                        println!("Clicked arrow button")
                    }
                    
                    ui.separator();
                    if ui.small_button("| Line").clicked() {
                        println!("Clicked line button")
                    }

                }

                ui.separator();
                if ui.small_button("Settings").clicked() {
                    println!("Settings button pressed");
                    self.view = Views::Settings;
                }
            });

            // Se c'è un'immagine nel buffer, mostrala nella main window
            if self.screenshot_image_buffer.is_some(){
                // È stato fatto uno screenshot il contenuto è dentro screenshot_image_buffer 
                ui.centered_and_justified(|ui| {
                    // Shouldn't be calling this here, read docs!
                    self.texture = Some(ui.ctx().load_texture(
                        "current_screenshot",
                        image_utils::load_image_from_memory(self.screenshot_image_buffer.clone().unwrap()),
                        Default::default(),
                    ));

                    ui.image(&self.texture.clone().unwrap());

                    // let texture = frame
                    // .tex_allocator()
                    // .alloc_srgba_premultiplied(size, &pixels);
                    // let size = egui::Vec2::new(size.0 as f32, size.1 as f32);
                    // self.texture = Some((size, texture));
                });


                // ui.centered_and_justified(add_contents)

            }

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

    pub fn settings_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            println!("settings_view");
            ui.label("Settings view");
            if ui.button("go back").clicked(){
                self.view = Views::Home;
            };
        });
    }
    
    pub fn screenshot_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("screenshot_view");

        // Prima hai scelto che screenshot fare, adesso fai lo screenshot

        if self.screenshot_type.is_some() {
            // It's not the screenshot, but the data describing it. It needs to be converted to an image.

            // quick and dirty solution, i wasted TOO much time on this...
            thread::sleep(time::Duration::from_millis(150));

            let (tx_screenshot_buffer, rx_screenshot_buffer) = mpsc::channel::<Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>>();
            let tmp_screenshot_type = self.screenshot_type.clone();

            // Take the screenshot and wait until it's done
            thread::spawn(move || {
                let screenshot_image_buffer = screenshot_utils::take_screenshot("png", tmp_screenshot_type);
                tx_screenshot_buffer.send(screenshot_image_buffer).unwrap();
            });

            
            self.screenshot_image_buffer = rx_screenshot_buffer.recv().unwrap();
            
            // save_utils::save_image(self.screenshot_image_buffer.clone().unwrap(), "screenshot.png", self.tx.clone());
            self.view = Views::Home;
            self.screenshot_type = None;
        }

        let width = _frame.info().window_info.monitor_size.unwrap().x;
        let height = _frame.info().window_info.monitor_size.unwrap().y;

        // Mostra UI per generare screenshot

        // Se l'utente non ha scelto che tipo di screenshot fare (tra FullScreen e PartialScreen)
        // Questa funzione viene chiamata ad ogni update() che può avvenire più volte in un secondo.
        // Siccome noi vogliamo che lo per chiamata di "screenshot_view", si fa un controllo con un contatore

        if self.screenshot_type.is_none() {
            _frame.set_visible(true);
            println!("Screenshot type is none");

            egui::Window::new("screenshot_view")
                .title_bar(false)
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    // self.id = Some(ui.layer_id());
                    ui.horizontal(|ui| {
                        ui.horizontal(|ui| {
                            
                            if ui.button("⛶").clicked() {
                                self.screenshot_type = Some(ScreenshotType::PartialScreen);
                                println!("PartialScreen button pressed");
                            }
                            ui.separator();

                            if ui.button("🖵").clicked() {
                                self.screenshot_type = Some(ScreenshotType::FullScreen);
                                println!("Fullscreen button pressed");
                            }
                            ui.separator();

                            if ui.button("◀").clicked() {
                                // restore_dim(&None, _frame, Some(Views::Home));
                                self.view = Views::Home;
                            }

                            if self.screenshot_type.is_some() {
                                // L'utente ha scelto che screenshot da fare
                                println!("scrernshot_type is some");
                                
                                // Hides the screen
                                _frame.set_visible(false);
                                ui.set_visible(false);
                                ctx.request_repaint();   
                            }

                            // TODO scommenta
                        });
                    });
                });
        }


    }
}