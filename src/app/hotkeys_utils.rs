// use egui_25::{KeyboardShortcut, Modifiers, Key};
use egui::{Key, KeyboardShortcut, Modifiers};

// Ho provato ad usare serde, ma i tipi KeyboardShortcut e Mdoifiers non sono serializzabili. Ho cercato di fare un wrapper e utilizzare
// https://stackoverflow.com/questions/56500357/how-do-i-reuse-code-for-similar-yet-distinct-types-in-rust <- questo
// Ma è troppo complicato e non ho tempo

// Prossimo tentativo -> quando si chiude l'applicazione scrivi su un file json i valori delle hotkeys e quando si apre l'applicazione caricali

#[derive(Debug, Copy, Clone)]
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

    pub fn check_if_valid(&self, shortcut: &KeyboardShortcut) -> (bool, String) {
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

    pub fn human_readable_shorcut(&self, field: &str) -> String {
        // Returns the shortcut into a human readable format 
        let shortcut = match field {
            "save" => self.save,
            "copy_to_clipboard" => self.copy_to_clipboard,
            "test" => self.test,
            "take_screenshot" => self.take_screenshot,
            _ => panic!("Invalid field name"),
        };

        let mut readable_shortcut = "".to_string();

        if shortcut.unwrap().modifiers.ctrl {
            readable_shortcut.push_str("CTRL+"); // cmd and ctrl are the same! (i hope)
        }

        if shortcut.unwrap().modifiers.alt {
            readable_shortcut.push_str("ALT+");
        }

        if shortcut.unwrap().modifiers.shift {
            readable_shortcut.push_str("SHIFT+");
        }

        readable_shortcut.push_str(shortcut.unwrap().key.name());

        return readable_shortcut;
    }

    pub fn from_name(self, key: &str) -> Key {
        
        let mut_key = key.to_ascii_uppercase();

        match mut_key.as_str() {
            "A" => Key::A,
            "B" => Key::B,
            "C" => Key::C,
            "D" => Key::D,
            "E" => Key::E,
            "F" => Key::F,
            "G" => Key::G,
            "H" => Key::H,
            "I" => Key::I,
            "J" => Key::J,
            "K" => Key::K,
            "L" => Key::L,
            "M" => Key::M,
            "N" => Key::N,
            "O" => Key::O,
            "P" => Key::P,
            "Q" => Key::Q,
            "R" => Key::R,
            "S" => Key::S,
            "T" => Key::T,
            "U" => Key::U,
            "V" => Key::V,
            "W" => Key::W,
            "X" => Key::X,
            "Y" => Key::Y,
            "Z" => Key::Z,
            "0" => Key::Num0,
            "1" => Key::Num1,
            "2" => Key::Num2,
            "3" => Key::Num3,
            "4" => Key::Num4,
            "5" => Key::Num5,
            "6" => Key::Num6,
            "7" => Key::Num7,
            "8" => Key::Num8,
            "9" => Key::Num9,
            _ => panic!("Invalid key"),
        }
    }

}
