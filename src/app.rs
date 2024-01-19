use egui::*;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use egui_modal::Modal;
use image::RgbaImage;
use arboard::Clipboard;
use std::sync::mpsc;
use std::{thread, time};

mod image_utils;
mod painting_utils;
mod pathlib;
mod save_utils;
mod screenshot_utils;
mod screenshot_view;
mod crop_lib;
mod hotkeys_utils;

use crate::app::save_utils::SavePath;

use self::save_utils::check_filename;

pub enum Views {
    Home,
    Settings,
    Screenshot,
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
    update_counter: u8,     // Serve per chiamare _frame.set_visible(). Una volta chiamato, la finestra diventa trasparente all'update successivo. Per questo motivo bisogna contare a quale update siamo arrivati.
    keyboard_shortcuts: hotkeys_utils::AllKeyboardShortcuts,
    clipboard: Option<Clipboard>,
    toasts: Toasts,
    new_shortcut: String,
    which_shortcut_field: String,
    modifier: Modifiers,
    key_var: String,
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
            update_counter: 0,
            keyboard_shortcuts: hotkeys_utils::AllKeyboardShortcuts::default(),
            clipboard: Clipboard::new().ok(),
            toasts: Toasts::new(),
            new_shortcut: "".to_string(),
            which_shortcut_field: "".to_string(),
            modifier: Modifiers::CTRL,
            key_var: "A".to_string(),
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
        if ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.test.unwrap())) {
            println!("Test shortcut pressed! Here's the other shortcuts");
            println!("{:?}", self.keyboard_shortcuts);
        }

        self.toasts.show(ctx);

        egui::CentralPanel::default().show(
            ctx,
            |ui| {
                // Some emojis can be used as icons!
                // https://docs.rs/egui/0.24.1/egui/special_emojis/index.html
                // https://www.egui.rs/#demo (Font Book)

                ui.horizontal(|ui| {
                    if ui.small_button("📷 Take Screenshot").clicked() || ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.take_screenshot.unwrap())){
                        self.view = Views::Screenshot;
                    }

                    if self.screenshot_image_buffer.is_some(){
                        // Se è stato fatto uno screenshot, mostra i bottoni per aggiungere modifiche e salvarlo

                        ui.separator();
                        if ui.small_button("💾 Save").clicked() || ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.save.unwrap())){
                            self.view = Views::Save;
                        }

                        ui.separator();
                        if ui.small_button("🗐 Copy to Clipboard").clicked() || ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.copy_to_clipboard.unwrap())){
                            if let Some(clip) = self.clipboard.as_mut() {
                                let image_buffer = self.painting.as_mut().unwrap().generate_rgba_image();

                                let ar_shitty_format =  arboard::ImageData {
                                    width: image_buffer.width() as usize,
                                    height: image_buffer.height() as usize,
                                    bytes: std::borrow::Cow::from(image_buffer.to_vec()),
                                };

                                if clip.set_image(ar_shitty_format).is_ok() { // <- that's what copies the image to the clipboard
                                    // println!("Copied to clipboard");
                                    self.toasts = Toasts::new()
                                        .anchor(Align2::CENTER_BOTTOM, (0.0, -20.0)) // 10 units from the bottom right corner
                                        .direction(egui::Direction::BottomUp);

                                    self.toasts.add(Toast {
                                        text: "Saved to clipboard!".into(),
                                        kind: ToastKind::Success,
                                        options: ToastOptions::default()
                                            .duration_in_seconds(3.0)
                                            .show_progress(true)
                                    });

                                } else {
                                    self.toasts = Toasts::new()
                                        .anchor(Align2::CENTER_BOTTOM, (0.0, -30.0)) // 10 units from the bottom right corner
                                        .direction(egui::Direction::BottomUp);

                                    self.toasts.add(Toast {
                                        text: "Error :(".into(),
                                        kind: ToastKind::Error,
                                        options: ToastOptions::default()
                                            .duration_in_seconds(3.0)
                                            .show_progress(true)
                                    });
                                }
                            }
                        }

                    }

                    ui.separator();
                    if ui.small_button("Settings").clicked() {
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
        if ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.test.unwrap())) {
            println!("{:?}", self.keyboard_shortcuts);
        }
        let modal = Modal::new(ctx, "Assign key modal");
        self.toasts.show(ctx);

        modal.show(|ui| {
            modal.title(ui, "Write a new shortcut");
            modal.frame(ui, |ui| {
                modal.body(ui, "Allowed values: A-Z, 0-9.");

                ui.separator();

                ui.add(widgets::text_edit::TextEdit::singleline(&mut self.key_var).char_limit(1).hint_text("A"));

                let tmp_modifier = if self.modifier == Modifiers::ALT {
                    "ALT"
                } else if self.modifier == Modifiers::CTRL {
                    "CTRL"
                } else {
                    "SHIFT"
                };

                ui.separator();

                egui::ComboBox::from_label("Select a modifier")
                .selected_text(tmp_modifier)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.modifier, Modifiers::ALT, "ALT");
                    ui.selectable_value(&mut self.modifier, Modifiers::CTRL, "CTRL");
                    ui.selectable_value(&mut self.modifier, Modifiers::SHIFT, "SHIFT");
                });

            modal.buttons(ui, |ui| {
                // After clicking, the modal is automatically closed
                if modal.button(ui, "close").clicked() {
                    // Actions after closing. Non servono per ora.
                };

                if ui.small_button("💾 Save").clicked() {
                    // Genera la shortcut, dopo controlla se è vera

                    let shortcut = KeyboardShortcut::new(self.modifier, self.keyboard_shortcuts.from_name(&self.key_var));

                    if self.keyboard_shortcuts.check_if_valid(&shortcut).0 {
                        // Shortcut valida -> rimpiazza
                        self.keyboard_shortcuts.update_keyboard_shortcut(&self.which_shortcut_field, shortcut);

                        self.toasts = Toasts::new()
                                        .anchor(Align2::CENTER_BOTTOM, (0.0, -20.0)) // 10 units from the bottom right corner
                                        .direction(egui::Direction::BottomUp);

                                    self.toasts.add(Toast {
                                        text: "Keyboard replaced succesfully!".into(),
                                        kind: ToastKind::Success,
                                        options: ToastOptions::default()
                                            .duration_in_seconds(3.0)
                                            .show_progress(true)
                                    });
                        
                        modal.close();
                        
                    } else {
                        // Shortcut non valida -> mostra errore

                        self.toasts = Toasts::new()
                                        .anchor(Align2::CENTER_BOTTOM, (0.0, -20.0)) // 10 units from the bottom right corner
                                        .direction(egui::Direction::BottomUp);

                                    self.toasts.add(Toast {
                                        text: format!("Keyboard shortcut already in use by action {:?}!", self.keyboard_shortcuts.check_if_valid(&shortcut).1).into(),
                                        kind: ToastKind::Error,
                                        options: ToastOptions::default()
                                            .duration_in_seconds(3.0)
                                            .show_progress(true)
                                    });
                    }
                }
            });

            }); 
        });

        // Will contain the shortcuts
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Settings view");

            if ui.button("Go back").clicked() {
                self.view = Views::Home;
            };

            pathlib::ui_settings(ui, &mut self.save_path);

            ui.separator();

            ui.push_id(2, |ui| {
                let mut table = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            // .cell_layout(Layout::left_to_right(egui::Align::Center))
            .column(egui_extras::Column::initial(150.0).clip(false).range(150.0..=300.0))
            .column(egui_extras::Column::auto().clip(false).range(100.0..=200.0))
            .column(egui_extras::Column::remainder().clip(false))
            .min_scrolled_height(0.0);

            table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Action");
                });
                header.col(|ui| {
                    ui.strong("Current Shortcut");
                });
                header.col(|ui| {
                    ui.strong("New Shortcut");
                });
            }).body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Show save view");
                        ui.label("Copy image to clipboard");
                        ui.label("Print shortcuts debug info");
                        ui.label("Take a screenshot");
                    });
                    row.col(|ui| {
                        ui.label(self.keyboard_shortcuts.human_readable_shorcut("save"));
                        ui.label(self.keyboard_shortcuts.human_readable_shorcut("copy_to_clipboard"));
                        ui.label(self.keyboard_shortcuts.human_readable_shorcut("test"));
                        ui.label(self.keyboard_shortcuts.human_readable_shorcut("take_screenshot"));
                    });
                    row.col(|ui| {
                        // let mut new_shortcut = "".to_string();
                        if ui.small_button("Edit").clicked() {
                            self.which_shortcut_field = "save".to_string();
                            modal.open();
                        }
                        if ui.small_button("Edit").clicked() {
                            self.which_shortcut_field = "copy_to_clipboard".to_string();
                            modal.open();
                        }
                        if ui.small_button("Edit").clicked() {
                            self.which_shortcut_field = "test".to_string();
                            modal.open();
                        }
                        if ui.small_button("Edit").clicked() {
                            self.which_shortcut_field = "take_screenshot".to_string();
                            modal.open();
                        }
                    });
                });
            });

            ui.separator();
            });

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
        // Prima hai scelto che screenshot fare, adesso fai lo screenshot
        // println!("screenshot_view");

        // Perché il contatore? Perché chiamare _frame.set_visible rendere trasparente la finestra soltanto al prossimo update. Se non controllassi
        // con un contatore, rimarrebbe la maschera nello screenshot perché non è stata nascosta.
        if self.screenshot_type.is_none() {
            self.update_counter = 0;
        } else {
            self.update_counter += 1;
        }

        // Questa parte è stata anticipata altrimenti si vedrebbe la maschera disegnata nelle righe successive
        self.screenshot_view.ui(ctx, _frame, &mut self.view, &mut self.screenshot_type);


        if self.screenshot_type.is_some() {
            // println!("Update counter {}", self.update_counter);
            // It's not the screenshot, but the data describing it. It needs to be converted to an image.

            if self.update_counter == 2 {
                
                // 150 è fisso perché nascondere la maschera richiede un po' di tempo.
                thread::sleep(time::Duration::from_millis(150 + self.screenshot_view.get_timer_delay() as u64));

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
                    //TODO\
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
                    // println!("default filename is: {}", self.save_path.name);
                }
                self.view = Views::Home;
                self.screenshot_type = None;
                self.painting = None;
                // _frame.set_window_size(vec2(640.0, 400.0));
                _frame.set_window_size(egui::Vec2::new(self.screenshot_image_buffer.as_mut().unwrap().width() as f32, self.screenshot_image_buffer.as_mut().unwrap().height() as f32));
                _frame.set_centered();
                _frame.set_visible(true);
                _frame.set_decorations(true); // Preparing for the next update in which we'll go back Home
            }

        }

        // Mostra UI per generare screenshot

        // Se l'utente non ha scelto che tipo di screenshot fare (tra FullScreen e PartialScreen)
        // Questa funzione viene chiamata ad ogni update() che può avvenire più volte in un secondo.
        // Siccome noi vogliamo che lo per chiamata di "screenshot_view", si fa un controllo con un contatore
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

