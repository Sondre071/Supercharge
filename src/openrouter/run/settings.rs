use crate::openrouter;
use crate::shared::menu::{self, Cursor};

pub fn settings() {
    let choice = {
        let Some((choice, _)) = menu::run(&mut Cursor::new(
            "OpenRouter settings",
            vec![""],
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
