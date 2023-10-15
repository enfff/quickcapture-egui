#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([600.0, 400.0].into()),
        min_window_size: Some([300.0, 200.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "Quickcapture",
        native_options,
        Box::new(|cc| Box::new(quickcapture::QuickCaptureApp::new(cc))),
    )
}