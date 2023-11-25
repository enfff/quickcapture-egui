/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state

use crate::components::initial_window::initial_window;
use crate::components::hidden_window::hidden_window;

use crate::app::Status::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status{
    InitialWindow,          // Initial state, empty window
    HiddenWindow,                 // Hide all windows
    // ScreenshotWindow,             // Screenshot taken
}

// impl Default for Status{
//     fn default() -> Self {
//         InitialWindow
//     }
// }

pub struct QuickCaptureApp {
    // Example stuff:
    // label: String,

    // #[serde(skip)] // This how you opt-out of serialization of a field

    // value: f32,
    status: Status,
}

impl Default for QuickCaptureApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            // label: "Hello World!".to_owned(),
            // value: 2.7,
            status: InitialWindow,
        }
    }
}

impl QuickCaptureApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }

    pub fn set_status(&mut self, status: Status){
        self.status = status;
    }
}

impl eframe::App for QuickCaptureApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
    

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui_extras::install_image_loaders(ctx);

        match self.status {
            InitialWindow => {
                initial_window(self, ctx, _frame);
            }
            HiddenWindow => {
                hidden_window(self, ctx, _frame);
            }
        }
    }    
}

// fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
//     ui.horizontal(|ui| {
//         ui.spacing_mut().item_spacing.x = 0.0;
//         ui.label("Powered by ");
//         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//         ui.label(" and ");
//         ui.hyperlink_to(
//             "eframe",
//             "https://github.com/emilk/egui/tree/master/crates/eframe",
//         );
//         ui.label(".");
//     });
// }
