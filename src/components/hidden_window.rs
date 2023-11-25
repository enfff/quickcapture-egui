
use eframe::egui;
use crate::app::QuickCaptureApp;


pub fn hidden_window(app: &mut QuickCaptureApp, ctx: &egui::Context, frame: &mut eframe::Frame){
    // This is a work around to compensate egui APIs. Infact, egui does not provide a way to hide all windows.   
    println!("Switched to Hidden Window");    
    frame.set_minimized(true);

    // Quite useless frankly, marked for deletion
}
