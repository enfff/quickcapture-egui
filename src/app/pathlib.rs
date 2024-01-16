use crate::app::save_utils::check_filename;
use crate::app::save_utils::SavePath;
use crate::app::ImgFormats;
use egui::{CollapsingHeader, Color32, ComboBox, ScrollArea, Ui};
use std::fs;

pub fn ui(ui: &mut Ui, path: &mut SavePath) {
    ui.label("Name");
    let response = ui.text_edit_singleline(&mut path.name);
    if !check_filename(&path.name) {
        ui.colored_label(
            Color32::LIGHT_RED,
            "Filename is not valid! Forbidden characters: \\ / : * ? \" < > |",
        );
    }

    if response.lost_focus() {
        println!("Name: {}", path.name);
        path.user_mod_name = true;
    }
    ui.end_row();
    ui.label("Format");
    ComboBox::from_label("")
        .selected_text(format!("{:?}", path.format))
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            ui.selectable_value(&mut path.format, ImgFormats::PNG, "PNG");
            ui.selectable_value(&mut path.format, ImgFormats::JPEG, "JPEG");
            ui.selectable_value(&mut path.format, ImgFormats::GIF, "GIF");
        });
    ui.end_row();
    let start_tree = path.path.clone();
    let scroll = ScrollArea::new([false, true]);
    ui.label(format!(
        "Destination Path: {}",
        path.path.clone().into_os_string().into_string().unwrap()
    ));

    scroll.show(ui, |ui| {
        CollapsingHeader::new("Select path to save screenshot")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(parent_dir) = start_tree.parent() {
                    if let Some(file_name) = parent_dir.file_name() {
                        if ui
                            .button(format!("🗁 {}", file_name.to_string_lossy()))
                            .clicked()
                        {
                            println!("Path: {}", parent_dir.display());
                            path.path = parent_dir.to_path_buf();
                        }
                    } else {
                        ui.colored_label(Color32::LIGHT_RED, "You can't go back!");
                    }
                }
                if let Some(file_name) = start_tree.file_name() {
                    CollapsingHeader::new(format!(
                        "🗁 {} (Current Path)",
                        file_name.to_string_lossy()
                    ))
                    .default_open(true)
                    .show(ui, |ui| {
                        if let Ok(entries) = fs::read_dir(&start_tree.clone()) {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    if entry.path().is_dir() {
                                        if ui
                                            .button(format!(
                                                "🗁 {}",
                                                entry.file_name().to_string_lossy()
                                            ))
                                            .clicked()
                                        {
                                            println!("Path: {}", entry.path().display());
                                            path.path = entry.path().to_path_buf();
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            });
    });
}
pub fn ui_settings(ui: &mut Ui, path: &mut SavePath) {
    ui.label("Format");
    ComboBox::from_label("")
        .selected_text(format!("{:?}", path.format))
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            ui.selectable_value(&mut path.format, ImgFormats::PNG, "PNG");
            ui.selectable_value(&mut path.format, ImgFormats::JPEG, "JPEG");
            ui.selectable_value(&mut path.format, ImgFormats::GIF, "GIF");
        });
    ui.end_row();
    let start_tree = path.path.clone();
    let scroll = ScrollArea::new([false, true]);
    ui.label(format!(
        "Destination Path: {}",
        path.path.clone().into_os_string().into_string().unwrap()
    ));

    scroll.show(ui, |ui| {
        CollapsingHeader::new("Select path to save screenshot")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(parent_dir) = start_tree.parent() {
                    if let Some(file_name) = parent_dir.file_name() {
                        if ui
                            .button(format!("🗁 {}", file_name.to_string_lossy()))
                            .clicked()
                        {
                            println!("Path: {}", parent_dir.display());
                            path.path = parent_dir.to_path_buf();
                        }
                    } else {
                        ui.colored_label(Color32::LIGHT_RED, "You can't go back!");
                    }
                }
                if let Some(file_name) = start_tree.file_name() {
                    CollapsingHeader::new(format!(
                        "🗁 {} (Current Path)",
                        file_name.to_string_lossy()
                    ))
                    .default_open(true)
                    .show(ui, |ui| {
                        if let Ok(entries) = fs::read_dir(&start_tree.clone()) {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    if entry.path().is_dir() {
                                        if ui
                                            .button(format!(
                                                "🗁 {}",
                                                entry.file_name().to_string_lossy()
                                            ))
                                            .clicked()
                                        {
                                            println!("Path: {}", entry.path().display());
                                            path.path = entry.path().to_path_buf();
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            });
    });
}
