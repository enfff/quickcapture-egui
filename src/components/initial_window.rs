
use eframe::egui;

use crate::app::Status;
use crate::app::QuickCaptureApp;

use screenshots::Screen;


pub fn initial_window(app: &mut QuickCaptureApp, ctx: &egui::Context, _frame: &mut eframe::Frame){
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {

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
        // ui.heading("eframe template");

        // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
        // if ui.button("Increment").clicked() {
        //     self.value += 1.0;
        // }

        // ui.separator();

        ui.horizontal(|ui| {
            if ui.small_button("📷 Screenshot").clicked() {
                println!("Screenshot button pressed");

                _frame.set_minimized(true); // Hides window

                let screens = Screen::all().unwrap();

                for screen in screens {
                    let image = screen.capture().unwrap();
                    image
                        .save("./target/33.png")
                        .unwrap();
                }

                // TODO Rendere il percorso valido per tutti i sistemi operativi
                // Docs: https://github.com/emilk/egui/blob/c69fe941afdea5ef6f3f84ed063554500b6262e8/eframe/examples/image.rs

                _frame.set_minimized(false); // Shows window
            }

            if ui.small_button("Switch to Hidden Window").clicked() {
                println!("Switch to Hidden Window button pressed");
                app.set_status(Status::HiddenWindow);
            }

            ui.add_space(4.0);

            if ui.small_button("💾 Save").clicked() {
                println!("Save button pressed")
            }

            ui.add_space(4.0);

            if ui.small_button("↖ Arrow").hovered() {
                println!("Hovering on arrow button")
            }

            ui.add_space(4.0);

            if ui.small_button("| Line").clicked() {
                println!("Pressed line button")
            }
        });

        // LOAD IMAGES

        // Add egui_extras as a dependency with the all_loaders feature.
        // Add a call to egui_extras::install_image_loaders in your app’s setup code.
        // Use Ui::image with some ImageSource.
        // https://docs.rs/egui/latest/egui/load/index.html

        ui.add(
            egui::Image::new(egui::include_image!("../../target/33.png"))
                .rounding(5.0)
        );
        
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