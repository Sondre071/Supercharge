use crate::openrouter;
use crate::shared::menu;

pub fn settings() {
    if let Some(result) = menu::run("OpenRouter settings", None, vec!["Select model", "Back"]) {
        match result {
            "Select model" => openrouter::ui::select_model(),
            _ => std::process::exit(0)
        }
    }
}
