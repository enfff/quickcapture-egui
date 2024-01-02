#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use tokio::runtime::Runtime;

mod app;
use app::QuickCaptureApp;
use crate::app::Views;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    //tokio runtime https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html
    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1000)).await;
            }
        })
    });

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
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
        egui_extras::install_image_loaders(ctx); // Used to load images

        //timer event receiver
        if let Ok(duration) = self.rx.try_recv() {
            println!("Time spent {:?}", duration);
            self.is_app_saving = false;
        }

        match self.view {
            Views::Home => {
                _frame.set_visible(true);
                _frame.set_decorations(true);
                self.home_view(ctx, _frame);
            },
            Views::Settings => self.settings_view(ctx, _frame),
            Views::Capture => {
                // Quando viene chiamata la finestra va mostrata
                // _frame.set_visible(false);
                self.screenshot_view(ctx, _frame)
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
