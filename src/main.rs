#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// use tokio::runtime::Runtime;

mod app;
use app::QuickCaptureApp;
use crate::app::Views;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        min_window_size: Some([300.0, 200.0].into()),
        initial_window_size: Some([640.0, 400.0].into()),
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(
        "QuickCapture (Preview Build)",
        native_options,
        Box::new(|cc| Box::new(app::QuickCaptureApp::new(cc))),
    )
}

impl eframe::App for QuickCaptureApp {

    // Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // egui_extras::install_image_loaders(ctx); // Used to load images
        
        match self.view {
            Views::Home => {
                _frame.set_visible(true);
                _frame.set_decorations(true);
                self.home_view(ctx, _frame);
            },
            Views::Capture => {
                _frame.set_decorations(false);
                self.screenshot_view(ctx, _frame);
            },
            Views::Settings => {
                self.settings_view(ctx, _frame)
            },
            Views::Save => {
                self.save_view(ctx, _frame);
            },
        }

    }

    // We're using this workaround to hide the window. This function is automatically
    // called whenever the requested view doesn't contain a CentralPanel.
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0).to_normalized_gamma_f32()
        egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
    }
}
