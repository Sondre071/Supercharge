use crate::openrouter;
use crate::shared::menu::{self, Cursor, NONE};

pub fn settings() {
    let choice = {
        let Some((choice, _)) = menu::run(&mut Cursor::new(
            "OpenRouter settings",
            NONE,
            vec!["Select model", "Back"],
        )) else {
            return;
        };
        
        choice
    };

    if choice.as_str() == "Select model" {
        openrouter::run::select_model();
    }
}
