use crate::openrouter;
use crate::utils::menu;

pub fn settings() {
    if let Some(result) = menu::run("OpenRouter settings", None, vec!["Select model", "Back"]) {
        match result {
            "Select model" => openrouter::run::select_model(),
            _ => openrouter::main(),
        }
    }
}
