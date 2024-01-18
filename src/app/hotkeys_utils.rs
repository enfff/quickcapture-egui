// use egui_25::{KeyboardShortcut, Modifiers, Key};
use egui::{KeyboardShortcut, Modifiers, Key};


// Ho provato ad usare serde, ma i tipi KeyboardShortcut e Mdoifiers non sono serializzabili. Ho cercato di fare un wrapper e utilizzare 
// https://stackoverflow.com/questions/56500357/how-do-i-reuse-code-for-similar-yet-distinct-types-in-rust <- questo
// Ma è troppo complicato e non ho tempo

#[derive(Debug)]
pub struct AllKeyboardShortcuts {
    pub save: Option<KeyboardShortcut>,
    pub copy_to_clipboard: Option<KeyboardShortcut>,
    pub test: Option<KeyboardShortcut>,
}

impl Default for AllKeyboardShortcuts {
    fn default() -> Self {
        Self {
            save: Some(KeyboardShortcut::new(Modifiers::CTRL, Key::S)),
            copy_to_clipboard: Some(KeyboardShortcut::new(Modifiers::CTRL, Key::C)),
            test: Some(KeyboardShortcut::new(Modifiers::CTRL, Key::T)),
        }
    }
}

impl AllKeyboardShortcuts {
    pub fn update_keyboard_shortcut(&mut self, field: &str, new_shortcut: KeyboardShortcut) {
        // This function assumems the shortcut is valid
        match field {
            "save" => self.save = Some(new_shortcut),
            "copy_to_clipboard" => self.copy_to_clipboard = Some(new_shortcut),
            _ => panic!("Invalid field name"),
        };
    }
}