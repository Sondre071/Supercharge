use crate::openrouter;
use crate::shared::menu;

pub fn settings() {
    if let Some(result) = menu::run(
        "OpenRouter settings",
        None,
        vec!["Select model", "Back"],
        None,
    ) {
        match result {
            "Select model" => openrouter::run::select_model(),
            _ => openrouter::main(),
        }
    }
}
