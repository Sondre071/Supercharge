use crate::openrouter;
use crate::shared::menu::{self, Menu};

pub fn settings() {
    let (choice, _) = menu::run(Menu::new(
        "OpenRouter settings",
        vec![""],
        vec!["Select model", "Back"],
    ))
    .unwrap();

    if choice.as_str() == "Select model" {
        openrouter::run::select_model();
    }
}
