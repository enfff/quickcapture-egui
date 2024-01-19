// use egui_25::{KeyboardShortcut, Modifiers, Key};
use egui::{Key, KeyboardShortcut, Modifiers};

// Ho provato ad usare serde, ma i tipi KeyboardShortcut e Mdoifiers non sono serializzabili. Ho cercato di fare un wrapper e utilizzare
// https://stackoverflow.com/questions/56500357/how-do-i-reuse-code-for-similar-yet-distinct-types-in-rust <- questo
// Ma è troppo complicato e non ho tempo

// Prossimo tentativo -> quando si chiude l'applicazione scrivi su un file json i valori delle hotkeys e quando si apre l'applicazione caricali

#[derive(Debug)]
pub struct AllKeyboardShortcuts {
    pub save: Option<KeyboardShortcut>,
    pub copy_to_clipboard: Option<KeyboardShortcut>,
    pub test: Option<KeyboardShortcut>,
    pub take_screenshot: Option<KeyboardShortcut>,
}

impl Default for AllKeyboardShortcuts {
    fn default() -> Self {
        Self {
            save: Some(KeyboardShortcut::new(Modifiers::CTRL, Key::S)),
            copy_to_clipboard: Some(KeyboardShortcut::new(Modifiers::CTRL, Key::C)),
            test: Some(KeyboardShortcut::new(Modifiers::CTRL, Key::T)),
            take_screenshot: Some(KeyboardShortcut::new(Modifiers::CTRL, Key::D)),
        }
    }
}

impl AllKeyboardShortcuts {
    pub fn update_keyboard_shortcut(&mut self, field: &str, new_shortcut: KeyboardShortcut) {
        // This function assumems the shortcut is valid, use check_if_valid to check if it is
        match field {
            "save" => self.save = Some(new_shortcut),
            "copy_to_clipboard" => self.copy_to_clipboard = Some(new_shortcut),
            "test" => self.test = Some(new_shortcut),
            "take_screenshot" => self.take_screenshot = Some(new_shortcut),
            _ => panic!("Invalid field name"),
        };
    }

    fn check_if_valid(&self, shortcut: &KeyboardShortcut) -> (bool, String) {
        // This function checks if the shortcut is valid, returns true|false and the name of the field
        // Using match gave too many issues, and so did implementing IntoIterator for AllKeyboardShortcuts.
        if shortcut.eq(self.save.as_ref().unwrap()) {
            return (false, "save".to_string());
        } else if shortcut.eq(self.copy_to_clipboard.as_ref().unwrap()) {
            return (false, "copy_to_clipboard".to_string());
        } else if shortcut.eq(self.test.as_ref().unwrap()) {
            return (false, "test".to_string());
        } else if shortcut.eq(self.take_screenshot.as_ref().unwrap()) {
            return (false, "take_screenshot".to_string());
        }

        return (true, "none".to_string());
    }
}
