
use eframe::egui;
use crate::app::QuickCaptureApp;
// use std::time::{Duration};
use std::thread::sleep;

pub fn hidden_window(app: &mut QuickCaptureApp, ctx: &egui::Context, _frame: &mut eframe::Frame){
    // This is a work around to compensate egui APIs. Infact, egui does not provide a way to hide all windows.   
    println!("Switched to Hidden Window");    
    _frame.set_minimized(true);
    // sleep(Duration::new(3,0));
    // _frame.set_minimized(false); // Shows window

    // Quite useless frankly, marked for deletion
}
